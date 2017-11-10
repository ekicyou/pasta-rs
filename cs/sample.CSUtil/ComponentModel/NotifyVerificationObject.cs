using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Runtime.CompilerServices;
using System.Runtime.Serialization;
using System.Xml.Serialization;
using System.Linq;

namespace System.ComponentModel
{
    [DataContract]
    public class NotifyVerificationObject : NotificationObjectEx, INotifyDataErrorInfo
    {
        [XmlIgnore]
        private Dictionary<string, IEnumerable<string>> CurrentErrors { get; } = new Dictionary<string, IEnumerable<string>>();

        /// <summary>
        /// 値を検証します。
        /// </summary>
        /// <param name="value"></param>
        /// <param name="propertyName"></param>
        protected void ValidateProperty(object value, [CallerMemberName]string propertyName = null)
        {
            ValidatePropertyInternal(value, propertyName);
        }

        internal void ValidatePropertyInternal(object value, string propertyName)
        {
            //ValidatePropertyInternal
            var context = new ValidationContext(this) { MemberName = propertyName };
            var validationErrors = new List<ValidationResult>();
            if (!Validator.TryValidateProperty(value, context, validationErrors))
            {
                var errors = validationErrors
                    .Select(error => error.ErrorMessage)
                    .ToArray();
                AddError(errors, propertyName);
            }
            else
            {
                RemoveError(propertyName);
            }
        }

        /// <summary>
        /// エラーを登録します。
        /// </summary>
        /// <param name="propertyName"></param>
        /// <param name="error"></param>
        protected void AddError(IEnumerable<string> errors, [CallerMemberName]string propertyName = null)
        {
            CurrentErrors[propertyName] = errors;
            OnErrorsChanged(propertyName);
        }

        /// <summary>
        /// エラーを解除します。
        /// </summary>
        /// <param name="propertyName"></param>
        protected void RemoveError([CallerMemberName]string propertyName = null)
        {
            CurrentErrors.Remove(propertyName);
            OnErrorsChanged(propertyName);
        }

        /// <summary>
        /// エラー状態が変更されたときにイベントを発行します。
        /// </summary>
        /// <param name="propertyName"></param>
        private void OnErrorsChanged(string propertyName)
        {
            var h = this.ErrorsChanged;
            if (h != null)
            {
                h(this, new DataErrorsChangedEventArgs(propertyName));
            }
        }

        /// <summary>エラー状態変更イベント</summary>
        public event EventHandler<DataErrorsChangedEventArgs> ErrorsChanged;

        /// <summary>
        /// エラー一覧を取得します。
        /// </summary>
        /// <param name="propertyName"></param>
        /// <returns></returns>
        public System.Collections.IEnumerable GetErrors(string propertyName)
        {
            if (string.IsNullOrEmpty(propertyName) ||
                !CurrentErrors.ContainsKey(propertyName))
                return null;

            return CurrentErrors[propertyName];
        }

        /// <summary>
        /// エラーが発生していればtrueを返します。
        /// </summary>
        public bool HasErrors => CurrentErrors.Count > 0;
    }
}