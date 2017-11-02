namespace CSUtil.Text.Csv
{
    /// <summary>
    /// CSVパーサインターフェース。
    /// </summary>
    public interface ICsvPushParser : ICsvParserBase
    {
        /// <summary>
        /// １行投入し、csv行が確定したら文字列の列挙を返す。
        /// </summary>
        /// <param name="line"></param>
        /// <returns>
        /// 確定した場合に文字列の列挙を返す。
        /// 未確定時はnull。
        /// </returns>
        string[] PushLine(string line);
    }
}