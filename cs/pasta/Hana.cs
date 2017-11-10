using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Pasta
{
    public class Hana : IDisposable
    {
        private static NLog.Logger Log { get; } = NLog.LogManager.GetCurrentClassLogger();

        /// <summary>オブジェクト開放時にキャンセルされる。</summary>
        public ICTSCancelDisposable CTS { get; private set; }

        /// <summary>shiori.dllのHInst。</summary>
        public IntPtr HInst { get; private set; }

        /// <summary>shiori.dllが存在するディレクトリ。load apiで渡される。</summary>
        public string LoadDir { get; private set; }

        /// <summary>
        /// shiori load apiのManage実装。
        /// </summary>
        /// <param name="hinst">shiori.dllのhinst</param>
        /// <param name="load_dir">shiori.dllが存在するディレクトリ。</param>
        /// <returns>正常終了時にtrue。</returns>
        public bool Load(IntPtr hinst, string load_dir)
        {
            try
            {
                Unload();
                CTS = new CancellationTokenSource().Begin();
                var ct = CTS.Token;
                HInst = hinst;
                LoadDir = load_dir;

                return true;
            }
            catch (Exception ex)
            {
                Log.Error(ex);
                return false;
            }
        }

        /// <summary>
        /// shiori unload apiのManage実装。
        /// </summary>
        /// <returns>正常終了時にtrue。</returns>
        public bool Unload()
        {
            try
            {
                if (CTS == null) return true;
                var cts = CTS;
                CTS = null;
                cts.Dispose();
                return true;
            }
            catch (Exception ex)
            {
                Log.Error(ex);
                return false;
            }
        }

        /// <summary>
        /// shiori request apiのManage実装。
        /// </summary>
        /// <param name="req">リクエスト文字列</param>
        /// <returns>レスポンス文字列</returns>
        public string Request(string req)
        {
            return string.Empty;
        }

        public void Dispose()
        {
            Unload();
        }
    }
}