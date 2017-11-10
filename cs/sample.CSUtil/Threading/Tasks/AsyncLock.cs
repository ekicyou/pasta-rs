using System;
using System.Threading;
using System.Threading.Tasks;

namespace CSUtil.Threading.Tasks
{
    /// <summary>
    /// 非同期実行用のロック。
    /// </summary>
    public sealed class AsyncLock
    {
        private readonly System.Threading.SemaphoreSlim m_semaphore
          = new System.Threading.SemaphoreSlim(1, 1);

        private readonly Task<IDisposable> m_releaser;

        public AsyncLock()
        {
            m_releaser = Task.FromResult((IDisposable)new Releaser(this));
        }

        public Task<IDisposable> LockAsync(CancellationToken ct)
        {
            ct.ThrowIfCancellationRequested();
            var wait = m_semaphore.WaitAsync(ct);
            return wait.IsCompleted ?
                    m_releaser :
                    wait.ContinueWith(
                      (_, state) => (IDisposable)state,
                      m_releaser.Result,
                      ct,
                      TaskContinuationOptions.ExecuteSynchronously,
                      TaskScheduler.Default
                    );
        }

        private sealed class Releaser : IDisposable
        {
            private readonly AsyncLock m_toRelease;

            internal Releaser(AsyncLock toRelease)
            {
                m_toRelease = toRelease;
            }

            public void Dispose()
            {
                m_toRelease.m_semaphore.Release();
            }
        }
    }
}