using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Threading;
using System.Windows.Forms;
using Pasta.Visuals;

namespace test_ui
{
    public partial class MainForm : Form
    {
        public MainForm()
        {
            InitializeComponent();
        }

        public ICTSCancelDisposable StageCTS { get; private set; }
        public CompositionStage Stage { get; private set; }

        private void DisposeStage()
        {
            var cts = StageCTS;
            StageCTS = null;
            cts?.Dispose();
        }

        private void CreateStage()
        {
            DisposeStage();
            Stage = new CompositionStage();
            ResizeStage();
        }

        private void ResizeStage()
        {
            Stage.CreateWindowResources(Handle, ClientSize.Width, ClientSize.Height, this.DeviceDpi);
        }

        private void MainForm_Shown(object sender, EventArgs e)
        {
            CreateStage();
        }

        private void MainForm_FormClosed(object sender, FormClosedEventArgs e)
        {
            DisposeStage();
        }

        private void MainForm_DpiChanged(object sender, DpiChangedEventArgs e)
        {
            ResizeStage();
        }

        private void MainForm_Resize(object sender, EventArgs e)
        {
            ResizeStage();
        }
    }
}