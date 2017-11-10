using Livet;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Serialization;

namespace System.ComponentModel
{
    [DataContract]
    public class NotificationObjectEx : NotificationObject
    {
        protected void RaisePropertyChanged(IEnumerable<string> propertyNames)
        {
            foreach (var propertyName in propertyNames)
            {
                RaisePropertyChanged(propertyName);
            }
        }

        protected void RaisePropertyChanged(string p1, string p2)
        {
            RaisePropertyChanged(new[] { p1, p2 });
        }

        protected void RaisePropertyChanged(string p1, string p2, params string[] pn)
        {
            RaisePropertyChanged(new[] { p1, p2 }.Concat(pn));
        }

        internal void NotificationStoreChanged(IEnumerable<string> propertyNames)
        {
            RaisePropertyChanged(propertyNames);
        }

        internal void NotificationStoreChanged(string propertyName)
        {
            RaisePropertyChanged(propertyName);
        }
    }
}