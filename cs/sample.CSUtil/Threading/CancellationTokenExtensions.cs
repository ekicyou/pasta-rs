using System.Runtime.InteropServices;

namespace System.Threading
{
    public static class CancellationTokenExtensions
    {
        /// <summary>
        /// CancelとDisposeを行います。
        /// </summary>
        /// <param name="cts"></param>
        public static void CancelAndDispose(this CancellationTokenSource cts)
        {
            if (cts == null) return;
            using (cts)
            {
                cts.Cancel();
            }
        }

        /// <summary>
        /// Dispose()が発生したらCancellationTokenSourceのCancel() ⇒ Dispose() を行うICTSCancelDisposableを作成します。
        /// </summary>
        /// <typeparam name="T"></typeparam>
        /// <param name="disposable"></param>
        /// <param name="token"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable Begin(this CancellationTokenSource cts)
        {
            return new CTSCancelDisposable(cts);
        }

        private class CTSCancelDisposable : ICTSCancelDisposable
        {
            private CancellationTokenSource CTS { get; set; }

            public void Dispose()
            {
                var cts = CTS;
                CTS = null;
                if (cts == null) return;
                if (cts.IsCancellationRequested) return;
                cts.Cancel();
                cts.Dispose();
            }

            public CTSCancelDisposable(CancellationTokenSource cts)
            {
                CTS = cts;
            }

            public CancellationToken Token
            {
                get
                {
                    if (CTS == null) throw new ObjectDisposedException("CancellationTokenSource");
                    if (CTS.IsCancellationRequested) throw new OperationCanceledException();
                    return CTS.Token;
                }
            }
        }

        /// <summary>
        /// CancellationToken によるキャンセル時にDisposeを呼び出します。
        /// </summary>
        /// <typeparam name="T"></typeparam>
        /// <param name="disposable"></param>
        /// <param name="ct"></param>
        /// <returns></returns>
        public static T RegisterBy<T>(this T disposable, CancellationToken ct) where T : IDisposable
        {
            if (disposable == null) return default(T);
            ct.Register(() => { using (disposable) { } });
            return disposable;
        }

        /// <summary>
        /// CancellationToken によるキャンセル時にMarshal.FreeCoTaskMemを呼び出します。
        /// </summary>
        /// <typeparam name="T"></typeparam>
        /// <param name="disposable"></param>
        /// <param name="ct"></param>
        /// <returns></returns>
        public static IntPtr FreeCoTaskMemBy(this IntPtr ptr, CancellationToken ct)
        {
            if (ptr == IntPtr.Zero) return ptr;
            ct.Register(() => Marshal.FreeCoTaskMem(ptr));
            return ptr;
        }

        /// <summary>
        /// CancelとDisposeを行うイベントに変換します。
        /// </summary>
        /// <param name="cts"></param>
        /// <returns></returns>
        public static EventHandler ToEventHandler(this CancellationTokenSource cts)
        {
            EventHandler h = (sender, args) => cts.CancelAndDispose();
            return h;
        }
    }
}