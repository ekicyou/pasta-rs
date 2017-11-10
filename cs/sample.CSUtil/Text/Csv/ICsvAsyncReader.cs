using System;
using System.Threading;
using System.Threading.Tasks;

namespace CSUtil.Text.Csv
{
    public interface ICsvAsyncReader : ICsvParserBase, IDisposable
    {
        /// <summary>
        /// CSV行を非同期に１行読込みます。
        /// </summary>
        /// <returns></returns>
        Task<string[]> ReadCsvLineAsync(CancellationToken ct);
    }
}