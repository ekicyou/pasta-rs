use crate::error::*;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::executor::LocalPool;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures::task::LocalSpawnExt;
use rhai::ImmutableString;

pub struct ScenePlayer {}
pub enum Request {
    Next,
    Cancel,
}

impl ScenePlayer {
    /// 終わるまでsinkにカットを返し続ける非同期関数
    pub fn start(
        &mut self,
        executor: &mut LocalPool,
    ) -> PastaResult<(Sender<Request>, Receiver<Option<ImmutableString>>)> {
        let (tx1, mut rx) = channel::<Request>(0);
        let (mut tx, rx1) = channel::<Option<ImmutableString>>(0);
        let spawner = executor.spawner();
        // 非同期タスク
        let future = async move {
            loop {
                // 何も返さないループ
                match rx.next().await {
                    Cancel => {
                        break;
                    }
                    Next => {
                        tx.send(None).await;
                        break;
                    }
                }
            }
        };
        // 登録と実行
        spawner.spawn_local(future)?;
        executor.run_until_stalled();

        Ok((tx1, rx1))
    }
}
