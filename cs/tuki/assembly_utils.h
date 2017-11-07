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
        /// �X�^�b�N���w�肵���K�w��k���ă��\�b�h�����擾���܂��B
        /// </summary>
        /// <param name="skipFrames">�k��J�E���g�B�O�̂Ƃ��Ăяo�������\�b�h�B</param>
        /// <returns></returns>
        static String^ GetMethodName(int skipFrames);

        /// <summary>
        /// �Ăяo�������\�b�h�̃��\�b�h�����擾���܂��B
        /// </summary>
        /// <returns></returns>
        static String^ GetMethodName();

        /// <summary>
        /// �w�肳�ꂽ�A�Z���u����Path��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <param name="assembly"></param>
        /// <returns></returns>
        static String^ GetAssemblyPath(Assembly^ assembly);

        /// <summary>
        /// �Ăяo�����̃A�Z���u����Path��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <returns></returns>
        static String^ GetCallingAssemblyPath();

        /// <summary>
        /// �w�肳�ꂽ�A�Z���u����DirecryPath��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <param name="assembly"></param>
        /// <returns></returns>
        static String^ GetAssemblyDirctory(Assembly^ assembly);

        /// <summary>
        /// �Ăяo�����̃A�Z���u���� DirecryPath��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <returns></returns>
        static String^ GetCallingAssemblyDirctory();

        /// <summary>
        /// �p�X���A�Z���u���Q�ƑΏۂɒǉ����܂��B
        /// </summary>
        /// <param name="path"></param>
        static IDisposable^ AddCurrentAssemblyResolvePath(String^ path);
    };
}