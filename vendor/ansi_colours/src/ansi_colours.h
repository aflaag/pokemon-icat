/* ansi_colours – true-colour ↔ ANSI terminal palette converter
   Copyright 2018 by Michał Nazarewicz <mina86@mina86.com>

   ansi_colours is free software: you can redistribute it and/or modify it
   under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation; either version 3 of the License, or (at
   your option) any later version.

   ansi_colours is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser
   General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with ansi_colours.  If not, see <http://www.gnu.org/licenses/>. */

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Returns sRGB colour corresponding to the index in the 256-colour ANSI
 * palette.  The colour is returned as 24-bit 0xrrggbb number.
 *
 * The first 16 colours (so-called system colours) are not standardised and
 * terminal emulators often allow them to be customised.  Because of this, their
 * value should not be relied upon.  For system colours, this function returns
 * default colours used by XTerm.
 *
 * Remaining 240 colours consist of a 6×6×6 colour cube and a 24-step greyscale
 * ramp.  Those are standardised and thus should be the same on every terminal
 * which supports 256-colour colour palette.
 *
 * For example:
 *
 *     assert(0x000000 == rgb_from_ansi256( 16));
 *     assert(0x5f87af == rgb_from_ansi256( 67));
 *     assert(0xffffff == rgb_from_ansi256(231));
 *     assert(0xeeeeee == rgb_from_ansi256(255));
 */
uint32_t rgb_from_ansi256(uint8_t index);


/* Returns index of a colour in 256-colour ANSI palette approximating given sRGB
 * colour.  The sRGB colour is expected in 24-bit 0xrrggbb format.  (Most
 * significant eight bits of the argument are ignored).
 *
 * Because the first 16 colours of the palette are not standardised and usually
 * user-configurable, the function essentially ignores them.
 *
 * For example:
 *
 *     assert( 16 == ansi256_from_rgb(0x000000));
 *     assert( 16 == ansi256_from_rgb(0x010101));
 *     assert( 16 == ansi256_from_rgb(0x000102));
 *     assert( 67 == ansi256_from_rgb(0x5f87af));
 *     assert(231 == ansi256_from_rgb(0xffffff));
 */
uint8_t ansi256_from_rgb(uint32_t rgb);

#ifdef __cplusplus
}
#endif
