using System.Collections.Generic;

namespace CSUtil.Math
{
    /// <summary>
    /// フィボナッチ数列の生成。
    /// </summary>
    public static class Fibonacci
    {
        public static IEnumerable<int> EnInt32()
        {
            var a = 0;
            var b = 1;
            yield return a;
            yield return b;
            while (true)
            {
                var fibo = a + b;
                yield return fibo;
                a = b;
                b = fibo;
            }
        }

        public static IEnumerable<long> EnInt64()
        {
            var a = 0L;
            var b = 1L;
            yield return a;
            yield return b;
            while (true)
            {
                var fibo = a + b;
                yield return fibo;
                a = b;
                b = fibo;
            }
        }
    }
}