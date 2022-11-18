get-ffmpeg:
	mkdir -p ffmpeg/bin
	mkdir -p temp
	curl https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz | tar x -C temp --strip-components=1
	mv temp/ffmpeg ./ffmpeg/bin/
	mv temp/ffprobe ./ffmpeg/bin/
	rm -rf temp
