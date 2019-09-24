﻿using System;
using System.Text;
using Berrysoft.XXTea;

namespace Berrysoft.Pages.Data
{
    public enum TeaCryptorType
    {
        Unselected,
        Tea,
        XTea,
        XXTea
    }

    public enum StringType
    {
        UTF8,
        Unicode,
        ASCII,
        GB2312,
        Base64
    }

    public class TeaService
    {
        public TeaService()
        {
            Encoding.RegisterProvider(CodePagesEncodingProvider.Instance);
        }

        public string? OriginalString { get; set; }
        public StringType OriginalStringType { get; set; }
        public string? KeyString { get; set; }
        public StringType KeyStringType { get; set; }
        public string? EncryptedString { get; set; }
        public StringType EncryptedStringType { get; set; }

        private TeaCryptorBase? cryptor;
        public TeaCryptorType CryptorType
        {
            get => cryptor switch
            {
                TeaCryptor _ => TeaCryptorType.Tea,
                XTeaCryptor _ => TeaCryptorType.XTea,
                XXTeaCryptor _ => TeaCryptorType.XXTea,
                _ => TeaCryptorType.Unselected
            };
            set => cryptor = value switch
            {
                TeaCryptorType.Tea => new TeaCryptor(),
                TeaCryptorType.XTea => new XTeaCryptor(),
                TeaCryptorType.XXTea => new XXTeaCryptor(),
                _ => null
            };
        }

        private byte[]? Encode(string? str, StringType type)
        {
            string notNullStr = str ?? string.Empty;
            return type switch
            {
                StringType.UTF8 => Encoding.UTF8.GetBytes(notNullStr),
                StringType.Unicode => Encoding.Unicode.GetBytes(notNullStr),
                StringType.ASCII => Encoding.ASCII.GetBytes(notNullStr),
                StringType.GB2312 => Encoding.GetEncoding("gb2312").GetBytes(notNullStr),
                StringType.Base64 => Convert.FromBase64String(notNullStr),
                _ => null
            };
        }

        private string? Decode(byte[]? data, StringType type)
        {
            byte[] notNullData = data ?? Array.Empty<byte>();
            return type switch
            {
                StringType.UTF8 => Encoding.UTF8.GetString(notNullData),
                StringType.Unicode => Encoding.Unicode.GetString(notNullData),
                StringType.ASCII => Encoding.ASCII.GetString(notNullData),
                StringType.GB2312 => Encoding.GetEncoding("gb2312").GetString(notNullData),
                StringType.Base64 => Convert.ToBase64String(notNullData),
                _ => null
            };
        }

        public void Encrypt()
        {
            if (cryptor != null)
            {
                try
                {
                    byte[]? inputData = Encode(OriginalString, OriginalStringType);
                    byte[] keyData = Encode(KeyString, KeyStringType) ?? Array.Empty<byte>();
                    if (inputData != null)
                    {
                        cryptor.ConsumeKey(keyData);
                        byte[] outputData = cryptor.Encrypt(inputData);
                        EncryptedString = Decode(outputData, EncryptedStringType);
                    }
                }
                catch (Exception e)
                {
                    EncryptedString = e.ToString();
                }
            }
        }

        public void Decrypt()
        {
            if (cryptor != null)
            {
                try
                {
                    byte[]? inputData = Encode(EncryptedString, EncryptedStringType);
                    byte[] keyData = Encode(KeyString, KeyStringType) ?? Array.Empty<byte>();
                    if (inputData != null)
                    {
                        cryptor.ConsumeKey(keyData);
                        byte[] outputData = cryptor.Decrypt(inputData);
                        OriginalString = Decode(outputData, OriginalStringType);
                    }
                }
                catch (Exception e)
                {
                    OriginalString = e.ToString();
                }
            }
        }
    }
}
