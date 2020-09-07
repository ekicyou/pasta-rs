use crate::error::*;
use crate::ss::builder::SakuraScriptBuilder;
pub use crate::ss::env::*;
use crate::ss::yield_redume;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::executor::block_on;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::task::{LocalSpawn, LocalSpawnExt};
use rhai::{Dynamic, Engine, EvalAltResult, FnPtr, ImmutableString, Map, RegisterFn, AST};
use std::cell::RefCell;
use std::rc::Rc;

/// シーン情報
#[derive(Debug, Default, Clone)]
pub struct Scene {
    fn_ptr: FnPtr,
}

#[derive(Debug)]
pub struct ScenePlayer {
    scene: Scene,
    req: EnvDic,
    actor: EnvDic,
    app: EnvDic,
    save: EnvDic,
    eng: Rc<RefCell<Engine>>,
    ast: Rc<RefCell<AST>>,
    builder: SakuraScriptBuilder,
    tx: Sender<Option<ImmutableString>>,
}

impl ScenePlayer {
    pub fn start<S: LocalSpawn>(
        spawner: S,
        scene: Scene,
        builder: SakuraScriptBuilder,
        eng: Rc<RefCell<Engine>>,
        ast: Rc<RefCell<AST>>,
    ) -> PastaResult<Receiver<Option<ImmutableString>>> {
        let (tx, rx) = channel::<Option<ImmutableString>>(0);
        let p = Self {
            scene: scene,
            req: Default::default(),
            actor: Default::default(),
            app: Default::default(),
            save: Default::default(),
            eng: eng,
            ast: ast,
            builder: builder,
            tx: tx,
        };
        let future = async move {
            if let Err(e) = p.schedule().await {
                log::error!("{}", e);
            }
        };
        spawner.spawn_local(future)?;

        Ok(rx)
    }

    async fn schedule(mut self) -> PastaResult<()> {
        // 初回待機
        self.tx.send(None).await?;
        self.tx.send(None).await?;

        Ok(())
    }

    async fn ayield<S: Into<ImmutableString>>(&mut self, s: S) -> PastaResult<()> {
        self.tx.send(Some(s.into())).await?;
        self.tx.send(None).await?;
        self.tx.send(None).await?;
        Ok(())
    }

    /// シーンをカットし、１シーン確定します。
    fn cut(&mut self) -> bool {
        let script = "スクリプト";
        true
    }
}
