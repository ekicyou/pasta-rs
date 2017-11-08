using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Setugekka
{
    /// <summary>
    /// SHIORI/3.0 インターフェース
    /// </summary>
    public interface IShiori3
    {
        /// <summary>shiori.dllのHInst。</summary>
        IntPtr HInst { get; }

        /// <summary>shiori.dllが存在するディレクトリ。load apiで渡される。</summary>
        string LoadDir { get; }

        /// <summary>
        /// shiori load apiのManage実装。
        /// </summary>
        /// <param name="hinst">shiori.dllのhinst</param>
        /// <param name="load_str">shiori.dllが存在するディレクトリ。</param>
        /// <returns>正常終了時にtrue。</returns>
        bool Load(IntPtr hinst, string load_str);

        /// <summary>
        /// shiori unload apiのManage実装。
        /// </summary>
        /// <returns>正常終了時にtrue。</returns>
        bool Unload();

        /// <summary>
        /// shiori request apiのManage実装。
        /// </summary>
        /// <param name="req">リクエスト文字列</param>
        /// <returns>レスポンス文字列</returns>
        string Request(string req);
    }
}