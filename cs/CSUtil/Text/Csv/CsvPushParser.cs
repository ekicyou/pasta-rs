using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

namespace CSUtil.Text.Csv
{
    /// <summary>
    /// CSVパーサ。
    /// １行ごとに読み込んだ文字列をプッシュし、確定したらcsv行を返します。
    /// </summary>
    public class CsvPushParser : ICsvPushParser
    {
        /// <summary>読込み行</summary>
        public int LineRow { get; private set; } = 0;

        /// <summary>CSV行</summary>
        public int CsvRow { get; private set; } = 0;

        private StringBuilder Buf { get; } = new StringBuilder();

        private int QuotCount { get; set; } = 0;

        /// <summary>
        /// １行投入し、csv行が確定したら文字列の列挙を返す。
        /// </summary>
        /// <param name="line"></param>
        /// <returns>
        /// 確定した場合に文字列の列挙を返す。
        /// 未確定時はnull。
        /// </returns>
        public string[] PushLine(string line)
        {
            if (line == null) return null;
            LineRow++;
            Buf.Append(line);
            QuotCount += line.Length
                - line.Replace("\"", string.Empty).Length;
            if ((QuotCount % 2) != 0) return null;

            // 確定時の処理
            QuotCount = 0;
            CsvRow++;
            if (Buf.Length == 0)
            {
                Buf.Clear();
                return new string[0];
            }
            Buf.Append(',');
            var rc = Commit(Buf.ToString());
            Buf.Clear();
            return rc;
        }

        /// <summary>
        /// 確定処理。
        /// </summary>
        /// <param name="line"></param>
        /// <returns></returns>
        private string[] Commit(string line)
        {
            var split = new List<string>();
            var qMatch = regexSplitter.Matches(line)
                .OfType<Match>();
            foreach (var match in qMatch)
            {
                var item = match.Value;
                split.Add(item.Substring(0, item.Length - 1).Trim()); // remove comma
            }

            if (trimDQuot)
            {
                for (int index = 0; index < split.Count; index++)
                {
                    string item = split[index];
                    if ((0 < item.Length) && (item[0] == '"') && (item[item.Length - 1] == '"'))
                    {
                        split[index] = item.Substring(1, System.Math.Max(0, item.Length - 2));
                    }
                }
            }

            if (formatEscapedDQuot)
            {
                for (int index = 0; index < split.Count; index++)
                {
                    split[index] = split[index].Replace("\"\"", "\"");
                }
            }

            return split.ToArray();
        }

        private const bool trimDQuot = true;
        private const bool formatEscapedDQuot = true;

        private static readonly string splitterPattern = "(\"(?:[^\"]|\"\")*\"|[^,]*),";

        private static readonly Regex regexSplitter =
            new Regex(splitterPattern, RegexOptions.Multiline);
    }
}