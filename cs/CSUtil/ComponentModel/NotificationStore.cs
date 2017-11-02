using System.Collections.Generic;
using System.Runtime.CompilerServices;

namespace System.ComponentModel
{
    /// <summary>
    /// 通知する値を管理する構造体。
    /// </summary>
    /// <typeparam name="T"></typeparam>
    public struct NotificationStore<T> : INotificationStore<T>
    {
        #region NotificationStore<T>

        /// <summary>コンパレータ</summary>
        private static readonly IEqualityComparer<T> EQ = EqualityComparer<T>.Default;

        #region 値の直接操作

        /// <summary>値。default(T)で初期化されます。</summary>
        private T store;

        /// <summary>
        /// コンストラクタ。
        /// </summary>
        /// <param name="initValue"></param>
        public NotificationStore(T initValue)
        {
            store = initValue;
        }

        /// <summary>
        /// 通知を伴わない値の強制変更。
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public void Reset(T value)
        {
            Value = value;
        }

        /// <summary>
        /// 通知を伴わない値の強制変更。
        /// 値の変更があるかどうかを返す。
        /// </summary>
        /// <param name="value"></param>
        /// <returns>値が変更されたならtrue</returns>
        public bool ResetAndCheck(T value)
        {
            if (EQ.Equals(Value, value)) return false;
            Reset(value);
            return true;
        }

        #endregion 値の直接操作

        #region Getter

        /// <summary>値プロパティ。</summary>
        private T Value
        {
            get { return store; }
            set { store = value; }
        }

        /// <summary>
        /// 値の取得。getterで利用してください。
        /// </summary>
        /// <returns></returns>
        public T Get()
        {
            return Value;
        }

        /// <summary>
        /// 値の取得。暗黙の型変換を利用します。
        /// </summary>
        /// <param name="store"></param>
        public static implicit operator T(NotificationStore<T> store)
        {
            return store.Value;
        }

        #endregion Getter

        #region Setter

        /// <summary>
        /// 値の設定。更新が行われればtrue。setterで利用してください。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        public bool Set<TO>(T value, TO THIS, [CallerMemberName]string propertyName = null)
            where TO : NotificationObjectEx
        {
            if (EQ.Equals(Value, value)) return false;
            Reset(value);
            THIS.NotificationStoreChanged(propertyName);
            return true;
        }

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
        public bool Set<TO>(T value, TO THIS, Action act, [CallerMemberName]string propertyName = null)
            where TO : NotificationObjectEx
        {
            if (!Set(value, THIS, propertyName)) return false;
            if (act != null) act();
            return true;
        }

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
        public bool Set<TO>(T value, TO THIS, IEnumerable<string> propertyNames, [CallerMemberName]string propertyName = null)
            where TO : NotificationObjectEx
        {
            if (!Set(value, THIS, propertyName)) return false;
            THIS.NotificationStoreChanged(propertyNames);
            return true;
        }

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
        public bool Set<TO>(T value, TO THIS, IEnumerable<string> propertyNames, Action act, [CallerMemberName]string propertyName = null)
            where TO : NotificationObjectEx
        {
            if (!Set(value, THIS, propertyNames, propertyName)) return false;
            if (act != null) act();
            return true;
        }

        #endregion Setter

        #region Validate

        /// <summary>
        /// 値の設定と検証。更新が行われればtrue。setterで利用してください。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        public bool V<TO>(T value, TO THIS, [CallerMemberName]string propertyName = null)
            where TO : NotifyVerificationObject
        {
            if (EQ.Equals(Value, value)) return false;
            Reset(value);
            THIS.ValidatePropertyInternal(value, propertyName);
            THIS.NotificationStoreChanged(propertyName);
            return true;
        }

        /// <summary>
        /// 値の設定と検証。更新が行われればtrue。
        /// 更新が行われた場合にactを実行します。
        /// 検証に失敗しても値は更新されます。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="act"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        public bool V<TO>(T value, TO THIS, Action act, [CallerMemberName]string propertyName = null)
            where TO : NotifyVerificationObject
        {
            if (!Set(value, THIS, propertyName)) return false;
            if (act != null) act();
            return true;
        }

        /// <summary>
        /// 値の設定と検証。更新が行われればtrue。
        /// 更新が行われた場合にpropertyNamesの更新を通知します。
        /// 検証に失敗しても値は更新されます。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyNames"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        public bool V<TO>(T value, TO THIS, IEnumerable<string> propertyNames, [CallerMemberName]string propertyName = null)
            where TO : NotifyVerificationObject
        {
            if (!Set(value, THIS, propertyName)) return false;
            THIS.NotificationStoreChanged(propertyNames);
            return true;
        }

        /// <summary>
        /// 値の設定と検証。更新が行われればtrue。
        /// 更新が行われた場合にpropertyNamesの更新を通知します。
        /// また、actを実行します。
        /// 検証に失敗しても値は更新されます。
        /// </summary>
        /// <typeparam name="TO"></typeparam>
        /// <param name="value"></param>
        /// <param name="THIS"></param>
        /// <param name="propertyNames"></param>
        /// <param name="act"></param>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        public bool V<TO>(T value, TO THIS, IEnumerable<string> propertyNames, Action act, [CallerMemberName]string propertyName = null)
            where TO : NotifyVerificationObject
        {
            if (!Set(value, THIS, propertyNames, propertyName)) return false;
            if (act != null) act();
            return true;
        }

        #endregion Validate

        #endregion NotificationStore<T>
    }
}