using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace CSUtil.Text.Csv
{
    public static class CsvWriteExt
    {
        /// <summary>
        /// 文字列の列挙を１行のcsvに変換して返します。
        /// </summary>
        /// <param name="items"></param>
        /// <returns></returns>
        public static string ToCsvLine(this IEnumerable<string> items)
        {
            var q = items
                .Select(EncloseDoubleQuotesIfNeed);
            return string.Join(",", q);
        }

        /// <summary>
        /// 必要ならば、文字列をダブルクォートで囲む
        /// </summary>
        public static string EncloseDoubleQuotesIfNeed(string field)
        {
            if (NeedEncloseDoubleQuotes(field))
            {
                return EncloseDoubleQuotes(field);
            }
            return field;
        }

        /// <summary>
        /// 文字列をダブルクォートで囲む
        /// </summary>
        private static string EncloseDoubleQuotes(string field)
        {
            if (field.IndexOf('"') > -1)
            {
                //"を""とする
                field = field.Replace("\"", "\"\"");
            }
            return "\"" + field + "\"";
        }

        /// <summary>
        /// 文字列をダブルクォートで囲む必要があるか調べる
        /// </summary>
        private static bool NeedEncloseDoubleQuotes(string field)
        {
            return field.IndexOf('"') > -1 ||
                field.IndexOf(',') > -1 ||
                field.IndexOf('\r') > -1 ||
                field.IndexOf('\n') > -1 ||
                field.StartsWith(" ") ||
                field.StartsWith("\t") ||
                field.EndsWith(" ") ||
                field.EndsWith("\t");
        }

        /// <summary>
        /// csv行を１行書き込みます。
        /// </summary>
        /// <param name="w"></param>
        /// <param name="items"></param>
        /// <returns></returns>
        public static void WriteCsvLine(this TextWriter w, IEnumerable<string> items)
        {
            var line = items.ToCsvLine();
            w.WriteLine(line);
        }

        /// <summary>
        /// csv行を１行非同期に書き込みます。
        /// </summary>
        /// <param name="w"></param>
        /// <param name="items"></param>
        /// <param name="ct"></param>
        /// <returns></returns>
        public static async Task WriteCsvLineAsync(this TextWriter w, IEnumerable<string> items, CancellationToken ct)
        {
            ct.ThrowIfCancellationRequested();
            var line = items.ToCsvLine();
            await w.WriteLineAsync(line);
        }
    }
}