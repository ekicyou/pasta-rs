using System;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Threading;

namespace CSUtil.Threading.Tasks
{
    public static class TaskSchedulerExt
    {
        public static Task<TaskScheduler> ToTaskSchedulerAsync(
            this Dispatcher dispatcher,
            DispatcherPriority priority = DispatcherPriority.Normal)
        {
            var taskCompletionSource = new TaskCompletionSource<TaskScheduler>();
            var invocation = dispatcher.BeginInvoke(new Action(() =>
              taskCompletionSource.SetResult(
                  TaskScheduler.FromCurrentSynchronizationContext())), priority);

            invocation.Aborted += (s, e) =>
                taskCompletionSource.SetCanceled();

            return taskCompletionSource.Task;
        }

        public static Task Run(this TaskFactory factory, Func<Task> func, CancellationToken ct)
        {
            factory.CancellationToken.ThrowIfCancellationRequested();
            return factory.StartNew(func, ct).Unwrap();
        }

        public static Task<T> Run<T>(this TaskFactory factory, Func<Task<T>> func, CancellationToken ct)
        {
            factory.CancellationToken.ThrowIfCancellationRequested();
            return factory.StartNew(func, ct).Unwrap();
        }

        public static Task Run(this TaskFactory factory, Action func, CancellationToken ct)
        {
            factory.CancellationToken.ThrowIfCancellationRequested();
            return factory.StartNew(func, ct);
        }

        public static Task<T> Run<T>(this TaskFactory factory, Func<T> func, CancellationToken ct)
        {
            factory.CancellationToken.ThrowIfCancellationRequested();
            return factory.StartNew(func, ct);
        }


    }
}