using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Threading;
using NLog;

namespace Setugekka.Hana
{
    public class Hana : IShiori3, IDisposable
    {
        private static ILogger Log { get; } = LogManager.GetCurrentClassLogger();
        private ICTSCancelDisposable CTS { get; set; }
        public CancellationToken CT => CTS != null ? CTS.Token : CancellationToken.None;

        public void Dispose()
        {
            using (var fn_cts = Log.TraceEnter())
            {
                var cts = CTS;
                CTS = null;
                cts?.Dispose();
            }
        }

        public IntPtr HInst { get; private set; }
        public string LoadDir { get; private set; }

        public bool Load(IntPtr hinst, string load_str)
        {
            using (var fn_cts = Log.TraceEnter())
            {
                Dispose();
                CTS = new CancellationTokenSource().Begin();
                HInst = hinst;
                LoadDir = load_str;
                return true;
            }
        }

        public bool Unload()
        {
            using (var fn_cts = Log.TraceEnter())
            {
                Dispose();
                return true;
            }
        }

        public string Request(string req)
        {
            using (var fn_cts = Log.TraceEnter())
            {
                return "SHIORI/3.0 204 No Content\r\nCharset: UTF-8\r\nX-Kanji-Test: 漢字の応答\r\n\r\n";
            }
        }
    }
}