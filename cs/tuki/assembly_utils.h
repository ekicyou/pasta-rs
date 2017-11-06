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

        Assembly^ Find(String^ path, String^ name)
        {
            try
            {
                auto assembly = Assembly::LoadFile(path);
                return assembly;
            }
            catch (Exception^)
            {
                return nullptr;
            }
        }

    public:
        Assembly^ CallBack(Object^ sender, ResolveEventArgs^ args)
        {
            Debug::WriteLine(String::Format("AssemblyResolve: [{0}]", args->Name));

            auto m = re->Match(args->Name);
            if (!m->Success) return nullptr;

            auto assemblyName = m->Groups[1]->Value;
            auto version = m->Groups[2]->Value;
            auto culture = m->Groups[3]->Value;
            auto publicKeyToken = m->Groups[4]->Value;

            Assembly^ rc;
            String^ path;

            path = String::Format("{0}\\{1}.dll", dir, assemblyName);
            rc = Find(path, args->Name);
            return rc;
        }

        AddCurrentAssemblyResolvePath_Deligate(AppDomain^ domain, String^ path)
        {
            dir = Path::GetFullPath(path);
            re = gcnew Regex(
                "^([^,]*), Version=([^,]*), Culture=([^,]*), PublicKeyToken=(.*)$",
                RegexOptions::Compiled);
            this->domain = domain;
            h = gcnew ResolveEventHandler(this, &AddCurrentAssemblyResolvePath_Deligate::CallBack);
            AddEvent();
        }
        ~AddCurrentAssemblyResolvePath_Deligate() {
            RemoveEvent();
        }
        void AddEvent() {
            domain->AssemblyResolve += h;
        }
        void RemoveEvent() {
            domain->AssemblyResolve -= h;
        }
    };

    static ref class AssemblyUtil
    {
    public:
        /// <summary>
        /// �X�^�b�N���w�肵���K�w��k���ă��\�b�h�����擾���܂��B
        /// </summary>
        /// <param name="skipFrames">�k��J�E���g�B�O�̂Ƃ��Ăяo�������\�b�h�B</param>
        /// <returns></returns>
        static String^ GetMethodName(int skipFrames)
        {
            auto sf = gcnew StackFrame(skipFrames + 1);
            auto m = sf->GetMethod();
            return m->ReflectedType->Name + "::" + m->Name;
        }

        /// <summary>
        /// �Ăяo�������\�b�h�̃��\�b�h�����擾���܂��B
        /// </summary>
        /// <returns></returns>
        static String^ GetMethodName()
        {
            return GetMethodName(1);
        }

        /// <summary>
        /// �w�肳�ꂽ�A�Z���u����Path��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <param name="assembly"></param>
        /// <returns></returns>
        static String^ GetAssemblyPath(Assembly^ assembly)
        {
            auto binURI = gcnew Uri(assembly->CodeBase);
            auto encPath = binURI->AbsolutePath;
            auto path = Uri::UnescapeDataString(encPath);
            auto fullPath = Path::GetFullPath(path);
            return fullPath;
        }

        /// <summary>
        /// �Ăяo�����̃A�Z���u����Path��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <returns></returns>
        static  String^ GetCallingAssemblyPath()
        {
            return GetAssemblyPath(Assembly::GetCallingAssembly());
        }

        /// <summary>
        /// �w�肳�ꂽ�A�Z���u����DirecryPath��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <param name="assembly"></param>
        /// <returns></returns>
        static  String^ GetAssemblyDirctory(Assembly^ assembly)
        {
            return Path::GetDirectoryName(GetAssemblyPath(assembly));
        }

        /// <summary>
        /// �Ăяo�����̃A�Z���u���� DirecryPath��Ԃ��܂��B
        /// �Ԃ�Path�̓V���h�[�R�s�[�ł͂Ȃ��A�{�̂�Path�ł��B
        /// </summary>
        /// <returns></returns>
        static String^ GetCallingAssemblyDirctory()
        {
            return GetAssemblyDirctory(Assembly::GetCallingAssembly());
        }

        /// <summary>
        /// �p�X���A�Z���u���Q�ƑΏۂɒǉ����܂��B
        /// </summary>
        /// <param name="path"></param>
        static IDisposable^ AddCurrentAssemblyResolvePath(String^ path)
        {
            auto cb = gcnew AddCurrentAssemblyResolvePath_Deligate(AppDomain::CurrentDomain, path);
            return cb;
        }
    };
}