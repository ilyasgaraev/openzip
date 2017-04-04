require "spec_helper"
require "climate_control"

describe Openzip do
  describe ".lib_ext" do
    before { stub_const("RUBY_PLATFORM", platform) }

    subject(:lib_ext) { described_class.lib_ext }

    context "when OS is Windows" do
      let(:platform) { "mswin32" }

      it "raises error" do
        expect { lib_ext }.to raise_error(NotImplementedError)
      end
    end

    context "when OS is Linux" do
      let(:platform) { "linux" }

      it { is_expected.to eq "so" }
    end

    context "when OS is OSX" do
      let(:platform) { "darwin" }

      it { is_expected.to eq "dylib" }
    end
  end

  describe ".extract" do
    subject(:extract_zip) { described_class.extract(zipfile, extract_path) }

    let(:zipfile) { "spec/fixtures/archive.zip" }
    let(:extract_path) { "tmp/extract/path" }

    it { is_expected.to be_truthy }

    it "creates new folder for extract path" do
      expect(File).to be_directory(extract_path)
    end

    it "escapes folder and file names" do
      expect(File).to be_directory("#{extract_path}/:..:test:folder")
      expect(File).to be_exist("#{extract_path}/!@#$%^&*()_+{}\"|'\\.:.txt")
    end

    it "extracts all files" do
      expect(Dir["#{extract_path}/*.txt"].length).to eq 4
      expect(Dir["#{extract_path}/:..:test:folder/*.txt"].length).to eq 1
    end

    describe "file content" do
      subject { Dir["#{extract_path}/*.txt"].map { |filename| File.read(filename) } }

      let(:expected_content) { %w[SpecialChars Test1 Test2 Test3] }

      it { is_expected.to match_array(expected_content) }
    end

    context "when zip file does NOT exist" do
      let(:zipfile) { "spec/fixtures/wrong.zip" }

      it { is_expected.to be_falsey }
    end

    context "when path to zip is a directory" do
      let(:zipfile) { "spec/fixtures" }

      it { is_expected.to be_falsey }
    end

    describe "debug mode" do
      let(:zipfile) { "spec/fixtures/wrong.zip" }

      specify { expect { extract_zip }.not_to output.to_stdout_from_any_process }

      context "when DEBUG env variable is defined" do
        around do |example|
          ClimateControl.modify DEBUG: debug_mode do
            example.run
          end
        end

        context "and equals true" do
          let(:debug_mode) { "true" }
          let(:error_message) { "Error: No such file or directory (os error 2)\n" }

          specify { expect { extract_zip }.to output(error_message).to_stdout_from_any_process }
        end

        context "and equals false" do
          let(:debug_mode) { "false" }

          specify { expect { extract_zip }.not_to output.to_stdout_from_any_process }
        end
      end
    end
  end
end
