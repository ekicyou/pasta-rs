using System;
using System.Runtime.CompilerServices;
using System.Threading;

namespace NLog
{
    public static class LoggerExt
    {
        private static ICTSCancelDisposable EnterLeaveLogAction(CancellationToken src, string name, Action<string> log_action)
        {
            src.ThrowIfCancellationRequested();
            var cts = new CancellationTokenSource().Begin();
            var ct = cts.Token;
            var prefix = $"[{name}]";
            var output = !string.IsNullOrWhiteSpace(name);
            if (output) log_action($"{prefix} >> 開始");
            var start = DateTime.Now;
            ct.Register(() =>
            {
                if (!output) return;
                log_action($"{prefix} << 終了 ...({(DateTime.Now - start).TotalSeconds:0.0}sec)");
            });
            if (src.CanBeCanceled)
            {
                src.Register(() => cts.Dispose()).RegisterBy(ct);
            }
            return cts;
        }

        #region TraceEnter

        /// <summary>
        /// using スコープの出入りを[Trace]レベルで通知します。
        /// 　・スコープIN ：「開始」
        /// 　・スコープOUT：「終了」
        /// 　・ICTSCancelDisposable を返します。
        /// 　・src がキャンセルされた場合、キャンセルされます。
        /// </summary>
        /// <typeparam name="TLogger"></typeparam>
        /// <param name="log"></param>
        /// <param name="src"></param>
        /// <param name="name"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable TraceEnter<TLogger>(this TLogger log, CancellationToken src, [CallerMemberName]string name = null)
            where TLogger : ILogger
        {
            return EnterLeaveLogAction(src, name, (text) => log.Trace(text));
        }

        /// <summary>
        /// using スコープの出入りを[Trace]レベルで通知します。
        /// 　・スコープIN ：「開始」
        /// 　・スコープOUT：「終了」
        /// 　・ICTSCancelDisposable を返します。
        /// </summary>
        /// <typeparam name="TLogger"></typeparam>
        /// <param name="log"></param>
        /// <param name="name"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable TraceEnter<TLogger>(this TLogger log, [CallerMemberName]string name = null)
            where TLogger : ILogger
        {
            return log.TraceEnter(CancellationToken.None, name);
        }

        #endregion TraceEnter

        #region DebugEnter

        /// <summary>
        /// using スコープの出入りを[Debug]レベルで通知します。
        /// 　・スコープIN ：「開始」
        /// 　・スコープOUT：「終了」
        /// 　・ICTSCancelDisposable を返します。
        /// 　・src がキャンセルされた場合、キャンセルされます。
        /// </summary>
        /// <typeparam name="TLogger"></typeparam>
        /// <param name="log"></param>
        /// <param name="src"></param>
        /// <param name="name"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable DebugEnter<TLogger>(this TLogger log, CancellationToken src, [CallerMemberName]string name = null)
            where TLogger : ILogger
        {
            return EnterLeaveLogAction(src, name, (text) => log.Debug(text));
        }

        /// <summary>
        /// using スコープの出入りを[Debug]レベルで通知します。
        /// 　・スコープIN ：「開始」
        /// 　・スコープOUT：「終了」
        /// 　・ICTSCancelDisposable を返します。
        /// </summary>
        /// <typeparam name="TLogger"></typeparam>
        /// <param name="log"></param>
        /// <param name="name"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable DebugEnter<TLogger>(this TLogger log, [CallerMemberName]string name = null)
            where TLogger : ILogger
        {
            return log.DebugEnter(CancellationToken.None, name);
        }

        #endregion DebugEnter

        #region InfoEnter

        /// <summary>
        /// using スコープの出入りを[Info]レベルで通知します。
        /// 　・スコープIN ：「開始」
        /// 　・スコープOUT：「終了」
        /// 　・ICTSCancelDisposable を返します。
        /// 　・src がキャンセルされた場合、キャンセルされます。
        /// </summary>
        /// <typeparam name="TLogger"></typeparam>
        /// <param name="log"></param>
        /// <param name="src"></param>
        /// <param name="name"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable InfoEnter<TLogger>(this TLogger log, CancellationToken src, [CallerMemberName]string name = null)
            where TLogger : ILogger
        {
            return EnterLeaveLogAction(src, name, (text) => log.Info(text));
        }

        /// <summary>
        /// using スコープの出入りを[Info]レベルで通知します。
        /// 　・スコープIN ：「開始」
        /// 　・スコープOUT：「終了」
        /// 　・ICTSCancelDisposable を返します。
        /// </summary>
        /// <typeparam name="TLogger"></typeparam>
        /// <param name="log"></param>
        /// <param name="name"></param>
        /// <returns></returns>
        public static ICTSCancelDisposable InfoEnter<TLogger>(this TLogger log, [CallerMemberName]string name = null)
            where TLogger : ILogger
        {
            return log.InfoEnter(CancellationToken.None, name);
        }

        #endregion InfoEnter
    }
}