using CSUtil.Math;
using System.Collections.Generic;

namespace System.Threading.Tasks
{
    /// <summary>
    /// フィボナッチ数列を利用した遅延処理を制御します。
    /// </summary>
    public class FibonacciDelay
    {
        private IEnumerator<TimeSpan> Gen { get; }

        /// <summary>
        /// コンストラクタ。
        /// </summary>
        /// <param name="rate"></param>
        /// <param name="max"></param>
        private FibonacciDelay(TimeSpan rate, TimeSpan max)
        {
            Gen = EnWaitTime(rate, max).GetEnumerator();
        }

        /// <summary>
        /// コンストラクタ。
        /// </summary>
        public FibonacciDelay() : this(TimeSpan.FromMilliseconds(1), TimeSpan.FromMilliseconds(100))
        {
        }

        /// <summary>
        /// 待機を行います。フィボナッチ数列に基づき遅延を増加します。
        /// </summary>
        /// <param name="ct"></param>
        /// <returns></returns>
        public async Task Delay(CancellationToken ct)
        {
            ct.ThrowIfCancellationRequested();
            Gen.MoveNext();
            var t = Gen.Current;
            if (t == TimeSpan.Zero) return;
            await Task.Delay(t, ct);
        }

        /// <summary>
        /// フィボナッチ数列を利用して待機時間を生成します。無限に繰り返します。
        /// </summary>
        /// <param name="offset"></param>
        /// <param name="max"></param>
        /// <returns></returns>
        private static IEnumerable<TimeSpan> EnWaitTime(TimeSpan rate, TimeSpan max)
        {
            var rateTick = rate.Ticks;
            foreach (var fib in Fibonacci.EnInt64())
            {
                var rc = rateTick * fib;
                var tick = TimeSpan.FromTicks(rc);
                if (tick > max) break;
                yield return tick;
            }
            while (true) { yield return max; }
        }
    }
}