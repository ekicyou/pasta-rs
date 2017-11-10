using System.Collections.Generic;
using System.Runtime.CompilerServices;

namespace System.ComponentModel
{
    /// <summary>
    /// 通知情報の管理。
    /// </summary>
    /// <typeparam name="T"></typeparam>
    public interface INotificationStore<T>
    {
        /// <summary>
        /// 値の取得。getterで利用してください。
        /// </summary>
        /// <returns></returns>
        T Get();

        /// <summary>
        /// 通知を伴わない値の強制変更。
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        void Reset(T value);

        /// <summary>
        /// 通知を伴わない値の強制変更。
        /// 値の変更があるかどうかを返す。
        /// </summary>
        /// <param name="value"></param>
        /// <returns>値が変更されたならtrue</returns>
        bool ResetAndCheck(T value);

        /// <summary>
        /// 値の設定。更新が行われればtrue。setterで利用してください。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        bool Set<TO>(T value, TO THIS, [CallerMemberName]string propertyName = null)
           where TO : NotificationObjectEx;

        /// <summary>
        /// 値の設定。更新が行われればtrue。
        /// 更新が行われた場合にactを実行します。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="act"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        bool Set<TO>(T value, TO THIS, Action act, [CallerMemberName]string propertyName = null)
           where TO : NotificationObjectEx;

        /// <summary>
        /// 値の設定。更新が行われればtrue。
        /// 更新が行われた場合にpropertyNamesの更新を通知します。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyNames"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        bool Set<TO>(T value, TO THIS, IEnumerable<string> propertyNames, [CallerMemberName]string propertyName = null)
           where TO : NotificationObjectEx;

        /// <summary>
        /// 値の設定。更新が行われればtrue。
        /// 更新が行われた場合にpropertyNamesの更新を通知します。
        /// また、actを実行します。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyNames"></param>
        /// <param name="act"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        bool Set<TO>(T value, TO THIS, IEnumerable<string> propertyNames, Action act, [CallerMemberName]string propertyName = null)
           where TO : NotificationObjectEx;
    }
}