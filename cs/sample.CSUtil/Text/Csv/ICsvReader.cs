using System;

namespace CSUtil.Text.Csv
{
    public interface ICsvReader : ICsvParserBase, IDisposable
    {
        /// <summary>
        /// CSV行を１行読込みます。
        /// </summary>
        /// <returns></returns>
        string[] ReadCsvLine();
    }
}