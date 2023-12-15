#include <signal.h>
#include <termios.h>
#include <sys/ioctl.h>
#include <stddef.h>


int cterm_get_sz(int *xc, int *yc, int *xpx, int *ypx) {
	struct winsize sz;

	if (-1 == ioctl(0, TIOCGWINSZ, &sz)) {
		return -1;
	}

	*xc = sz.ws_col;
	*yc = sz.ws_row;
	*xpx = sz.ws_xpixel;
	*ypx = sz.ws_ypixel;

	return 0;
}

int cterm_set_sigwinch_callback(void (*handler)(int)) {
	struct sigaction sa;

	sigemptyset(&sa.sa_mask);
	sa.sa_flags = 0;
	sa.sa_handler = handler;
	if (-1 == sigaction(SIGWINCH, &sa, NULL))
		return -1;
	
	return 0;
}




