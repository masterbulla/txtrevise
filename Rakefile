#
# Rakefile for building txtrevise on Travis CI.
#

task :travis => [:build, :install, :test]

task :build do
	Dir.chdir("python") do
		sh "make freeze"
		puts ""
	end
end

task :install do
	Dir.chdir("python") do
		sh "sudo make instnix"
		puts ""
	end
end

task :test do
	Dir.chdir("python") do
		sh "make testnix"
		puts ""
	end
end

task :clean do
	Dir.chdir("python") do
		sh "make cleannix"
		puts ""
	end
end #
