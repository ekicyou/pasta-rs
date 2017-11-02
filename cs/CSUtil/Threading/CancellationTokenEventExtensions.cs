namespace System.Threading
{
    public static class CancellationTokenEventExtensions
    {
        /// <summary>
        /// イベントを登録し、キャンセル時に登録解除するように設定します。
        /// </summary>
        /// <typeparam name="TARGS"></typeparam>
        /// <param name="ct"></param>
        /// <param name="addAction"></param>
        /// <param name="removeAction"></param>
        /// <param name="h"></param>
        /// <returns></returns>
        public static CancellationTokenRegistration AddEvent<TARGS>(
            this CancellationToken ct,
            Action<Action<object, TARGS>> addAction,
            Action<Action<object, TARGS>> removeAction,
            Action<object, TARGS> h)
            where TARGS : EventArgs
        {
            addAction(h);
            return ct.Register(() => removeAction(h));
        }
    }
}