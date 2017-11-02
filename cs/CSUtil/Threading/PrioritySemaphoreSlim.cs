using System.Collections.Concurrent;
using System.Linq;
using System.Threading.Tasks;

namespace System.Threading
{
    /// <summary>
    /// SemaphoreSlim(1,1)を利用した優先度付きWaitを実装します。
    /// </summary>
    public class PrioritySemaphoreSlim
    {
        private readonly SemaphoreSlim m_semaphore
            = new SemaphoreSlim(1, 1);

        public int PriorityLimit { get; }

        private struct WaitItem
        {
            public TaskCompletionSource<bool> Src;
            public IDisposable CancelDisposable;
        }

        private ConcurrentQueue<WaitItem>[] PriorityQueue { get; }

        private CancellationToken CancelToken;

        /// <summary>
        /// 管理するプライオリティ値を設定したプライオリティセマフォを作成します。
        /// </summary>
        /// <param name="priorityLimit"></param>
        public PrioritySemaphoreSlim(int priorityLimit, CancellationToken ct)
        {
            PriorityLimit = priorityLimit;
            PriorityQueue = Enumerable.Range(0, priorityLimit)
                .Select(a => new ConcurrentQueue<WaitItem>())
                .ToArray();
            CancelToken = ct;
        }

        /// <summary>
        /// セマフォをリリースします。
        /// </summary>
        public void Release()
        {
            m_semaphore.Release();
        }

        /// <summary>
        /// 指定優先度のキューで待機します。
        /// </summary>
        /// <param name="priority"></param>
        /// <param name="ct"></param>
        /// <returns></returns>
        public Task WaitAsync(int priority, CancellationToken ct)
        {
            if (priority < 0) throw new ArgumentOutOfRangeException("priority < 0");
            if (priority >= PriorityLimit) throw new ArgumentOutOfRangeException($"priority >= {PriorityLimit}");
            ct.ThrowIfCancellationRequested();
            // １つキューに登録する
            var src = new TaskCompletionSource<bool>();
            var item = new WaitItem
            {
                Src = src,
                CancelDisposable = ct.Register(() => src.TrySetCanceled()),
            };
            PriorityQueue[priority].Enqueue(item);

            // １つキューから取り出す処理をタスクに積む
            WaitOne();

            // 戻り値を返す
            return item.Src.Task;
        }

        private async void WaitOne()
        {
            await m_semaphore.WaitAsync(CancelToken);
            while (true)
            {
                foreach (var queue in PriorityQueue)
                {
                    if (queue.IsEmpty) continue;
                    WaitItem item;
                    if (queue.TryDequeue(out item))
                    {
                        using (item.CancelDisposable)
                        {
                            item.Src.TrySetResult(true);
                            return;
                        }
                    }
                }
            }
        }
    }
}