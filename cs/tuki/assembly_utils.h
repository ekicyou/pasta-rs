#pragma once

using namespace System;
using namespace System::Diagnostics;
using namespace System::IO;
using namespace System::Reflection;
using namespace System::Threading;
using namespace System::Text::RegularExpressions;

namespace Setugekka {
    ref class AddCurrentAssemblyResolvePath_Deligate
    {
    private:
        String^ dir;
        Regex^ re;
        AppDomain^ domain;
        ResolveEventHandler^ h;

        Assembly^ Find(String^ path, String^ name);

    public:
        Assembly^ CallBack(Object^ sender, ResolveEventArgs^ args);
        AddCurrentAssemblyResolvePath_Deligate(AppDomain^ domain, String^ path);
        ~AddCurrentAssemblyResolvePath_Deligate();
        void AddEvent();
        void RemoveEvent();
    };

    public ref class AssemblyUtil
    {
    public:
        /// <summary>
        /// スタックを指定した階層を遡ってメソッド名を取得します。
        /// </summary>
        /// <param name="skipFrames">遡るカウント。０のとき呼び出したメソッド。</param>
        /// <returns></returns>
        static String^ GetMethodName(int skipFrames);

        /// <summary>
        /// 呼び出し元メソッドのメソッド名を取得します。
        /// </summary>
        /// <returns></returns>
        static String^ GetMethodName();

        /// <summary>
        /// 指定されたアセンブリのPathを返します。
        /// 返すPathはシャドーコピーではなく、本体のPathです。
        /// </summary>
        /// <param name="assembly"></param>
        /// <returns></returns>
        static String^ GetAssemblyPath(Assembly^ assembly);

        /// <summary>
        /// 呼び出し元のアセンブリのPathを返します。
        /// 返すPathはシャドーコピーではなく、本体のPathです。
        /// </summary>
        /// <returns></returns>
        static String^ GetCallingAssemblyPath();

        /// <summary>
        /// 指定されたアセンブリのDirecryPathを返します。
        /// 返すPathはシャドーコピーではなく、本体のPathです。
        /// </summary>
        /// <param name="assembly"></param>
        /// <returns></returns>
        static String^ GetAssemblyDirctory(Assembly^ assembly);

        /// <summary>
        /// 呼び出し元のアセンブリの DirecryPathを返します。
        /// 返すPathはシャドーコピーではなく、本体のPathです。
        /// </summary>
        /// <returns></returns>
        static String^ GetCallingAssemblyDirctory();

        /// <summary>
        /// パスをアセンブリ参照対象に追加します。
        /// </summary>
        /// <param name="path"></param>
        static IDisposable^ AddCurrentAssemblyResolvePath(String^ path);
    };
}