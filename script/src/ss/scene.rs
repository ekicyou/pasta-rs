use crate::error::*;
use crate::ss::yield_redume;
use futures::task::{LocalSpawn, LocalSpawnExt};
use rhai::ImmutableString;
use std::iter::Iterator;

#[derive(Debug)]
pub struct ScenePlayer {
    yy: yield_redume::Yield<ImmutableString>,
    scene: Scene,
}

#[derive(Debug, Default, Clone)]
pub struct Scene {}

impl ScenePlayer {
    pub fn start<S: LocalSpawn>(
        spawner: &S,
        scene: Scene,
    ) -> PastaResult<yield_redume::Resume<ImmutableString>> {
        let (yy, rr) = yield_redume::yield_redume::<ImmutableString>();
        let future = async move {
            match yy.start().await {
                Some(yy) => {
                    let player = ScenePlayer {
                        yy: yy,
                        scene: scene,
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

    async fn schedule(mut self) {
        loop {
            // 何かする

            // yieldして待機
            if (!self.yy("test").await) {
                return;
            }

            // 試験実装なのでそのまま終了
            return;
        }
    }

    /// シーンを再生します。
    async fn action_scene(&mut self) {
        return;
    }
}

impl Iterator for ScenePlayer {
    type Item = ImmutableString;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/*
pub struct ScenePlayer {
    tx: Sender<Option<ImmutableString>>,
    rx: Receiver<Request>,
}
impl ScenePlayer {
    /// 終わるまでsinkにカットを返し続ける非同期関数
    pub fn start(
        executor: &mut LocalPool,
    ) -> PastaResult<(Sender<Request>, Receiver<Option<ImmutableString>>)> {
        let (tx1, rx) = channel::<Request>(0);
        let (tx, rx1) = channel::<Option<ImmutableString>>(0);
        let spawner = executor.spawner();
        let player = Self::new(tx, rx);
        spawner.spawn_local(player.service())?;
        executor.run_until_stalled();

        Ok((tx1, rx1))
    }

    fn new(tx: Sender<Option<ImmutableString>>, rx: Receiver<Request>) -> ScenePlayer {
        ScenePlayer { tx: tx, rx: rx }
    }

    /// 非同期タスク
    async fn service(mut self) {
        loop {
            // 何も返さないループ
            match self.rx.next().await {
                Cancel => {
                    break;
                }
                Next => {
                    self.tx.send(None).await;
                    break;
                }
            }
        }
    }
}
*/
