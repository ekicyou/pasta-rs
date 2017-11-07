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
        bool Load(string load_str);

        bool Unload();

        string Request(string req);
    }
}