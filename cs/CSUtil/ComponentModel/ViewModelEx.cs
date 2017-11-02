using Livet.Messaging;
using System.Xml.Serialization;

namespace System.ComponentModel
{
    [Serializable]
    public abstract class ViewModelEx : NotifyVerificationObject
    {
        /// <summary>
        /// このViewModelクラスの基本Messegerインスタンスです。
        /// </summary>
        [XmlIgnore]
        public InteractionMessenger Messenger
        {
            get
            {
                if (_messenger == null)
                {
                    _messenger = new InteractionMessenger();
                }
                return _messenger;
            }
            set
            {
                _messenger = value;
            }
        }

        [NonSerialized]
        private InteractionMessenger _messenger;
    }
}