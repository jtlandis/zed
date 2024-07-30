use std::sync::Arc;

use anyhow::Result;
use collections::{HashMap, HashSet};
use command_palette_hooks::CommandPaletteFilter;
use gpui::{AppContext, Context, EntityId, Global, Model, ModelContext, Subscription, Task, View};
use language::Language;
use project::Fs;
use settings::{Settings, SettingsStore};

use crate::kernels::{kernel_specifications, KernelProcess};
use crate::{JupyterSettings, KernelSpecification, Session};

struct GlobalReplStore(Model<ReplStore>);

impl Global for GlobalReplStore {}

pub struct ReplStore {
    fs: Arc<dyn Fs>,
    enabled: bool,
    sessions: HashMap<EntityId, View<Session>>,
    kernel_specifications: Vec<KernelSpecification>,
    _subscriptions: Vec<Subscription>,
}

impl ReplStore {
    const NAMESPACE: &'static str = "repl";

    pub(crate) fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
        let store = cx.new_model(move |cx| Self::new(fs, cx));

        store
            .update(cx, |store, cx| store.refresh_kernelspecs(cx))
            .detach_and_log_err(cx);

        cx.set_global(GlobalReplStore(store))
    }

    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalReplStore>().0.clone()
    }

    pub fn new(fs: Arc<dyn Fs>, cx: &mut ModelContext<Self>) -> Self {
        let subscriptions = vec![cx.observe_global::<SettingsStore>(move |this, cx| {
            this.set_enabled(JupyterSettings::enabled(cx), cx);
        })];

        let this = Self {
            fs,
            enabled: JupyterSettings::enabled(cx),
            sessions: HashMap::default(),
            //kernels: HashMap::default(),
            kernel_specifications: Vec::new(),
            _subscriptions: subscriptions,
        };
        this.on_enabled_changed(cx);
        this
    }

    pub fn fs(&self) -> &Arc<dyn Fs> {
        &self.fs
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn kernel_specifications(&self) -> impl Iterator<Item = &KernelSpecification> {
        self.kernel_specifications.iter()
    }

    pub fn sessions(&self) -> impl Iterator<Item = &View<Session>> {
        self.sessions.values()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut ModelContext<Self>) {
        if self.enabled == enabled {
            return;
        }

        self.enabled = enabled;
        self.on_enabled_changed(cx);
    }

    fn on_enabled_changed(&self, cx: &mut ModelContext<Self>) {
        if !self.enabled {
            CommandPaletteFilter::update_global(cx, |filter, _cx| {
                filter.hide_namespace(Self::NAMESPACE);
            });

            return;
        }

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.show_namespace(Self::NAMESPACE);
        });

        cx.notify();
    }

    pub fn refresh_kernelspecs(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let kernel_specifications = kernel_specifications(self.fs.clone());
        cx.spawn(|this, mut cx| async move {
            let kernel_specifications = kernel_specifications.await?;

            this.update(&mut cx, |this, cx| {
                this.kernel_specifications = kernel_specifications;
                cx.notify();
            })
        })
    }

    pub fn kernelspec(
        &self,
        language: &Language,
        cx: &mut ModelContext<Self>,
    ) -> Option<KernelSpecification> {
        let settings = JupyterSettings::get_global(cx);
        let language_name = language.code_fence_block_name();
        let selected_kernel = settings.kernel_selections.get(language_name.as_ref());

        self.kernel_specifications
            .iter()
            .find(|runtime_specification| {
                if let Some(selected) = selected_kernel {
                    // Top priority is the selected kernel
                    runtime_specification.name.to_lowercase() == selected.to_lowercase()
                } else {
                    // Otherwise, we'll try to find a kernel that matches the language
                    runtime_specification.kernelspec.language.to_lowercase()
                        == language_name.to_lowercase()
                }
            })
            .cloned()
    }

    pub fn get_session(&self, entity_id: EntityId) -> Option<&View<Session>> {
        self.sessions.get(&entity_id)
    }

    pub fn insert_session(&mut self, entity_id: EntityId, session: View<Session>) {
        self.sessions.insert(entity_id, session);
    }

    pub fn remove_session(&mut self, entity_id: EntityId) {
        self.sessions.remove(&entity_id);
    }

    pub fn get_language_kernel_single(&self, language: &Language, cx: &AppContext) -> SingleKernel {
        let lang = language.name().to_lowercase();
        let mut kernels = self
            .sessions
            .iter()
            .map(|(_entity_id, session)| {
                let s_ref = session.read(cx);
                let res = s_ref
                    .kernel_specification
                    .kernelspec
                    .language
                    .to_lowercase()
                    == lang;
                let kernel = match &s_ref.kernel {
                    crate::Kernel::RunningKernel(kernel) => Some(kernel.process.clone()),
                    _ => None,
                };
                (res, kernel)
            })
            .filter(|(x, y)| x & y.is_some())
            .map(|(_, x)| x.unwrap())
            .collect::<HashSet<_>>()
            .into_iter();
        let kernel = if let Some(kernel_proc) = kernels.next() {
            if let Some(_) = kernels.next() {
                return SingleKernel::ErrorMultiple;
            }
            kernel_proc.clone()
        } else {
            return SingleKernel::ErrorNone;
        };
        SingleKernel::Single(kernel)
    }
}

pub enum SingleKernel {
    Single(Model<KernelProcess>),
    ErrorMultiple,
    ErrorNone,
}
/*
impl std::error::Error for SingleKernelError {}

impl fmt::Display for SingleKernelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SingleKernelError::Multiple(language) => write!(
                f,
                "Expected one Kernel for language {}, found multiple",
                language
            ),
            SingleKernelError::None(language) => write!(
                f,
                "Expected one Kernel for language {}, found none",
                language
            ),
        }
    }
}
*/
