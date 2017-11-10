namespace test_ui
{
    partial class MainForm
    {
        /// <summary>
        /// 必要なデザイナー変数です。
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// 使用中のリソースをすべてクリーンアップします。
        /// </summary>
        /// <param name="disposing">マネージ リソースを破棄する場合は true を指定し、その他の場合は false を指定します。</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows フォーム デザイナーで生成されたコード

        /// <summary>
        /// デザイナー サポートに必要なメソッドです。このメソッドの内容を
        /// コード エディターで変更しないでください。
        /// </summary>
        private void InitializeComponent()
        {
            this.logArea = new System.Windows.Forms.TextBox();
            this.SuspendLayout();
            // 
            // logArea
            // 
            this.logArea.BackColor = System.Drawing.Color.White;
            this.logArea.BorderStyle = System.Windows.Forms.BorderStyle.None;
            this.logArea.Dock = System.Windows.Forms.DockStyle.Fill;
            this.logArea.Enabled = false;
            this.logArea.Location = new System.Drawing.Point(0, 0);
            this.logArea.Multiline = true;
            this.logArea.Name = "logArea";
            this.logArea.ReadOnly = true;
            this.logArea.Size = new System.Drawing.Size(320, 480);
            this.logArea.TabIndex = 0;
            // 
            // MainForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.AutoSize = true;
            this.BackColor = System.Drawing.Color.White;
            this.ClientSize = new System.Drawing.Size(320, 480);
            this.Controls.Add(this.logArea);
            this.Name = "MainForm";
            this.Text = "テスト画面";
            this.FormClosed += new System.Windows.Forms.FormClosedEventHandler(this.MainForm_FormClosed);
            this.Shown += new System.EventHandler(this.MainForm_Shown);
            this.DpiChanged += new System.Windows.Forms.DpiChangedEventHandler(this.MainForm_DpiChanged);
            this.Resize += new System.EventHandler(this.MainForm_Resize);
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.TextBox logArea;
    }
}

