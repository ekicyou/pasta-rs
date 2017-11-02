using System;
using System.Threading;
using System.Threading.Tasks;

namespace CSUtil.Threading.Tasks
{
    public class PriorityAsyncLock
    {
        private PrioritySemaphoreSlim m_semaphore { get; }

        private readonly Task<IDisposable> m_releaser;

        public PriorityAsyncLock(int priorityLimit, CancellationToken ct)
        {
            m_semaphore = new PrioritySemaphoreSlim(priorityLimit, ct);
            m_releaser = Task.FromResult((IDisposable)new Releaser(this));
        }

        public Task<IDisposable> LockAsync(int priority, CancellationToken ct)
        {
            ct.ThrowIfCancellationRequested();
            var wait = m_semaphore.WaitAsync(priority, ct);
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
            private readonly PriorityAsyncLock m_toRelease;

            internal Releaser(PriorityAsyncLock toRelease)
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