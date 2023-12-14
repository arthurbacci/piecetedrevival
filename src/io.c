#include <stdio.h>
#include <sys/ioctl.h>

struct cterm_sz {
	int row;
	int col;
	int pxwidth;
	int pxheight;
};

struct cterm_sz cterm_get_sz() {
	struct winsize sz;
	ioctl(0, TIOCGWINSZ, &sz);
	struct cterm_sz r = {sz.ws_row, sz.ws_col, sz.ws_xpixel, sz.ws_ypixel};
	return r;
}




