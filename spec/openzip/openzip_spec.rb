require "spec_helper"

describe Openzip do
  describe ".extract" do
    let(:zipfile) { "spec/fixtures/hello.zip" }
    let(:extract_path) { "#{Dir.tmpdir}/extract/path" }

    before { described_class.extract(zipfile, extract_path) }

    it "creates all folders from extract path" do
      expect(File).to be_directory(extract_path)
    end

    it "extracts all files to folder" do
      expect(Dir["#{extract_path}/*.txt"].length).to eq 3
    end
  end
end
