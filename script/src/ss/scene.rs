use crate::error::*;
use crate::ss::yield_redume;
use futures::executor::block_on;
use futures::task::{LocalSpawn, LocalSpawnExt};
use rhai::{FnPtr, ImmutableString};

pub use crate::ss::env::*;

/// シーン情報
#[derive(Debug, Default, Clone)]
pub struct Scene {
    fn_ptr: FnPtr,
}

#[derive(Debug)]
pub struct ScenePlayer {
    yy: yield_redume::Yield<ImmutableString>,
    scene: Scene,
    env: PlayEnv,
}

impl ScenePlayer {
    pub fn start<S: LocalSpawn>(
        spawner: &S,
        env: PlayEnv,
        scene: Scene,
    ) -> PastaResult<yield_redume::Resume<ImmutableString>> {
        let (yy, rr) = yield_redume::yield_redume::<ImmutableString>();
        let future = async move {
            match yy.start().await {
                Some(yy) => {
                    let player = ScenePlayer {
                        yy: yy,
                        scene: scene,
                        env: env,
                    };
                    player.schedule().await;
                }
                _ => (),
            }
        };
        spawner.spawn_local(future)?;

        Ok(rr)
    }

    async fn yy<S: Into<ImmutableString>>(&mut self, s: S) -> bool {
        self.yy.yield_async(s.into()).await
    }

    async fn schedule(mut self) {}

    /// シーンをカットし、１シーン確定します。
    fn cut(&mut self) -> bool {
        let script = "スクリプト";

        block_on(async move { self.yy(script).await })
    }
}
