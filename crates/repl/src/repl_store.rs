use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use collections::HashMap;
use command_palette_hooks::CommandPaletteFilter;
use gpui::{
    prelude::*, AppContext, EntityId, Global, Model, ModelContext, Subscription, Task, View,
};
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
    kernels: HashMap<String, Model<KernelProcess>>,
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
            kernels: HashMap::default(),
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

    pub fn insert_langauge(&mut self, language: String, file_path: PathBuf) {
        self.kernel_files.insert(language, file_path);
    }

    pub fn remove_session(&mut self, entity_id: EntityId) {
        self.sessions.remove(&entity_id);
    }

    pub fn get_language_session(&self, language: String) -> Option<PathBuf> {
        match self.kernel_files.get(&language) {
            Some(path) => Some(path.clone()),
            None => None,
        }
    }

    pub fn get_language_session_single(
        &self,
        language: &Language,
        cx: &AppContext,
    ) -> Result<View<Session>, String> {
        //Iter<EntityId, View<Session>>
        //info!("attempting to compare against {}", language.name());
        let mut sessions = self.sessions.iter().filter(|(_entity_id, session)| {
            let passed_lang = language.name();
            /*info!(
            "session lang: {}",
            session.read(cx).kernel_specification.kernelspec.language
            );*/
            *session.read(cx).kernel_specification.kernelspec.language == *passed_lang
        });
        let session = if let Some((_entity_id, session)) = sessions.next() {
            if let Some((_entity_id, _session)) = sessions.next() {
                return Err(format!(
                    "expected one session with langauge {}, found multiple",
                    language.name()
                ));
            }
            session.clone()
        } else {
            return Err(format!("No sessions with language {}", language.name()));
        };
        Ok(session)
    }
}
