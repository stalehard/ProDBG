#include "pd_backend.h"
#include "pd_host.h"
#include "pd_menu.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/*
0x0000e003      4cc1e0         jmp 0xe0c1
0x0000e006      40             rti
0x0000e007      40             rti
0x0000e008      40             rti
0x0000e009      40             rti
0x0000e00a      40             rti
0x0000e00b      4a             lsr a
0x0000e00c      4a             lsr a
0x0000e00d      4e4e4e         lsr 0x4e4e
0x0000e010      4e5757         lsr 0x5757
0x0000e013      5757           sre 0x57,x
0x0000e015      5761           sre 0x61,x
0x0000e017      6767           rra 0x67
0x0000e019      6464           nop 0x64
0x0000e01b      08             php
0x0000e01c      0500           ora 0x00
0x0000e01e      00             brk
0x0000e01f      00             brk
0x0000e020      416c           eor (0x6c,x)
0x0000e022      6578           adc 0x78
0x0000e024      616e           adc (0x6e,x)
0x0000e026      6465           nop 0x65
0x0000e028      72             hlt
0x0000e029      205769         jsr 0x6957
0x0000e02c      6b6c           arr #0x6c
0x0000e02e      756e           adc 0x6e,x
0x0000e030      6420           nop 0x20
0x0000e032      28             plp
0x0000e033      5769           sre 0x69,x
0x0000e035      6b6c           arr #0x6c
0x0000e037      756e           adc 0x6e,x
0x0000e039      6429           nop 0x29
0x0000e03b      202020         jsr 0x2020
0x0000e03e      20209d         jsr 0x9d20
0x0000e041      69e3           adc #0xe3
0x0000e043      bd58e3         lda 0xe358,x
0x0000e046      9d68e3         sta 0xe368,x
0x0000e049      60             rts
0x0000e04a      9da8e3         sta 0xe3a8,x
0x0000e04d      60             rts
0x0000e04e      a000           ldy #0x00
0x0000e050      8cfce0         sty 0xe0fc
0x0000e053      8df8e0         sta 0xe0f8
0x0000e056      60             rts
0x0000e057      8d7fe3         sta 0xe37f
0x0000e05a      8d86e3         sta 0xe386
0x0000e05d      8d8de3         sta 0xe38d
0x0000e060      60             rts
0x0000e061      4c62e2         jmp 0xe262
0x0000e064      98             tya
0x0000e065      f050           beq 0x00e0b7
0x0000e067      b9c5e5         lda 0xe5c5,y
0x0000e06a      85ff           sta 0xff
0x0000e06c      bd68e3         lda 0xe368,x
0x0000e06f      c902           cmp #0x02
0x0000e071      901d           bcc 0x00e090
0x0000e073      f032           beq 0x00e0a7
0x0000e075      bc8113         ldy 0x1381,x
0x0000e078      bd9513         lda 0x1395,x
0x0000e07b      f9b513         sbc 0x13b5,y
0x0000e07e      48             pha
0x0000e07f      bd9613         lda 0x1396,x
0x0000e082      f90e14         sbc 0x140e,y
0x0000e085      a8             tay
0x0000e086      68             pla
0x0000e087      b017           bcs 0x00e0a0
0x0000e089      65fe           adc 0xfe
0x0000e08b      98             tya
0x0000e08c      65ff           adc 0xff
0x0000e08e      1027           bpl 0x00e0b7
0x0000e090      bd95e3         lda 0xe395,x
0x0000e093      65fe           adc 0xfe
0x0000e095      9d95e3         sta 0xe395,x
0x0000e098      bd96e3         lda 0xe396,x
0x0000e09b      65ff           adc 0xff
0x0000e09d      4c5fe2         jmp 0xe25f
0x0000e0a0      e5fe           sbc 0xfe
0x0000e0a2      98             tya
0x0000e0a3      e5ff           sbc 0xff
0x0000e0a5      3010           bmi 0x00e0b7
0x0000e0a7      bd95e3         lda 0xe395,x
0x0000e0aa      e5fe           sbc 0xfe
0x0000e0ac      9d95e3         sta 0xe395,x
0x0000e0af      bd96e3         lda 0xe396,x
0x0000e0b2      e5ff           sbc 0xff
0x0000e0b4      4c5fe2         jmp 0xe25f
0x0000e0b7      bc81e3         ldy 0xe381,x
0x0000e0ba      4c56e2         jmp 0xe256
0x0000e0bd      8dc4e0         sta 0xe0c4
0x0000e0c0      60             rts
0x0000e0c1      a200           ldx #0x00
0x0000e0c3      a000           ldy #0x00
0x0000e0c5      3030           bmi 0x00e0f7
0x0000e0c7      8a             txa
0x0000e0c8      a229           ldx #0x29
0x0000e0ca      9d53e3         sta 0xe353,x
0x0000e0cd      ca             dex
0x0000e0ce      10fa           bpl 0x00e0ca
0x0000e0d0      8d15d4         sta 0xd415
0x0000e0d3      8d46e1         sta 0xe146
0x0000e0d6      8df8e0         sta 0xe0f8
0x0000e0d9      8ec4e0         stx 0xe0c4
0x0000e0dc      aa             tax
0x0000e0dd      20e7e0         jsr 0xe0e7
0x0000e0e0      a207           ldx #0x07
0x0000e0e2      20e7e0         jsr 0xe0e7
0x0000e0e5      a20e           ldx #0x0e
0x0000e0e7      a905           lda #0x05
0x0000e0e9      9d7fe3         sta 0xe37f,x
0x0000e0ec      a901           lda #0x01
0x0000e0ee      9d80e3         sta 0xe380,x
0x0000e0f1      9d82e3         sta 0xe382,x
0x0000e0f4      4c49e3         jmp 0xe349
0x0000e0f7      a000           ldy #0x00
0x0000e0f9      f045           beq 0x00e140
0x0000e0fb      a900           lda #0x00
0x0000e0fd      d023           bne 0x00e122
0x0000e0ff      b996e5         lda 0xe596,y
0x0000e102      f012           beq 0x00e116
0x0000e104      1019           bpl 0x00e11f
0x0000e106      0a             asl a
0x0000e107      8d4be1         sta 0xe14b
0x0000e10a      b9ade5         lda 0xe5ad,y
0x0000e10d      8d46e1         sta 0xe146
0x0000e110      b997e5         lda 0xe597,y
0x0000e113      d01f           bne 0x00e134
0x0000e115      c8             iny
0x0000e116      b9ade5         lda 0xe5ad,y
0x0000e119      8d41e1         sta 0xe141
0x0000e11c      4c31e1         jmp 0xe131
0x0000e11f      8dfce0         sta 0xe0fc
0x0000e122      b9ade5         lda 0xe5ad,y
0x0000e125      18             clc
0x0000e126      6d41e1         adc 0xe141
0x0000e129      8d41e1         sta 0xe141
0x0000e12c      cefce0         dec 0xe0fc
0x0000e12f      d011           bne 0x00e142
0x0000e131      b997e5         lda 0xe597,y
0x0000e134      c9ff           cmp #0xff
0x0000e136      c8             iny
0x0000e137      98             tya
0x0000e138      9003           bcc 0x00e13d
0x0000e13a      b9ade5         lda 0xe5ad,y
0x0000e13d      8df8e0         sta 0xe0f8
0x0000e140      a900           lda #0x00
0x0000e142      8d16d4         sta 0xd416
0x0000e145      a900           lda #0x00
0x0000e147      8d17d4         sta 0xd417
0x0000e14a      a900           lda #0x00
0x0000e14c      090f           ora #0x0f
0x0000e14e      8d18d4         sta 0xd418
0x0000e151      205be1         jsr 0xe15b
0x0000e154      a207           ldx #0x07
0x0000e156      205be1         jsr 0xe15b
0x0000e159      a20e           ldx #0x0e
0x0000e15b      de80e3         dec 0xe380,x
0x0000e15e      f00b           beq 0x00e16b
0x0000e160      1006           bpl 0x00e168
0x0000e162      bd7fe3         lda 0xe37f,x
0x0000e165      9d80e3         sta 0xe380,x
0x0000e168      4c13e2         jmp 0xe213
0x0000e16b      bc58e3         ldy 0xe358,x
0x0000e16e      b906e0         lda 0xe006,y
0x0000e171      8d08e2         sta 0xe208
0x0000e174      8d11e2         sta 0xe211
0x0000e177      bd56e3         lda 0xe356,x
0x0000e17a      d030           bne 0x00e1ac
0x0000e17c      bc7de3         ldy 0xe37d,x
0x0000e17f      b96ee4         lda 0xe46e,y
0x0000e182      85fe           sta 0xfe
0x0000e184      b971e4         lda 0xe471,y
0x0000e187      85ff           sta 0xff
0x0000e189      bc53e3         ldy 0xe353,x
0x0000e18c      b1fe           lda (0xfe),y
0x0000e18e      c9ff           cmp #0xff
0x0000e190      9006           bcc 0x00e198
0x0000e192      c8             iny
0x0000e193      b1fe           lda (0xfe),y
0x0000e195      a8             tay
0x0000e196      b1fe           lda (0xfe),y
0x0000e198      c9e0           cmp #0xe0
0x0000e19a      9008           bcc 0x00e1a4
0x0000e19c      e9f0           sbc #0xf0
0x0000e19e      9d54e3         sta 0xe354,x
0x0000e1a1      c8             iny
0x0000e1a2      b1fe           lda (0xfe),y
0x0000e1a4      9d7ee3         sta 0xe37e,x
0x0000e1a7      c8             iny
0x0000e1a8      98             tya
0x0000e1a9      9d53e3         sta 0xe353,x
0x0000e1ac      bc82e3         ldy 0xe382,x
0x0000e1af      b9e6e4         lda 0xe4e6,y
0x0000e1b2      9dace3         sta 0xe3ac,x
0x0000e1b5      bd6ae3         lda 0xe36a,x
0x0000e1b8      f053           beq 0x00e20d
0x0000e1ba      38             sec
0x0000e1bb      e960           sbc #0x60
0x0000e1bd      9d81e3         sta 0xe381,x
0x0000e1c0      a900           lda #0x00
0x0000e1c2      9d68e3         sta 0xe368,x
0x0000e1c5      9d6ae3         sta 0xe36a,x
0x0000e1c8      bd58e3         lda 0xe358,x
0x0000e1cb      c903           cmp #0x03
0x0000e1cd      f03e           beq 0x00e20d
0x0000e1cf      b9f1e4         lda 0xe4f1,y
0x0000e1d2      9d6ce3         sta 0xe36c,x
0x0000e1d5      fe83e3         inc 0xe383,x
0x0000e1d8      b9d0e4         lda 0xe4d0,y
0x0000e1db      f008           beq 0x00e1e5
0x0000e1dd      9d6de3         sta 0xe36d,x
0x0000e1e0      a900           lda #0x00
0x0000e1e2      9d6ee3         sta 0xe36e,x
0x0000e1e5      b9dbe4         lda 0xe4db,y
0x0000e1e8      f008           beq 0x00e1f2
0x0000e1ea      8df8e0         sta 0xe0f8
0x0000e1ed      a900           lda #0x00
0x0000e1ef      8dfce0         sta 0xe0fc
0x0000e1f2      b9c5e4         lda 0xe4c5,y
0x0000e1f5      9d6be3         sta 0xe36b,x
0x0000e1f8      b9bae4         lda 0xe4ba,y
0x0000e1fb      9da8e3         sta 0xe3a8,x
0x0000e1fe      b9afe4         lda 0xe4af,y
0x0000e201      9da7e3         sta 0xe3a7,x
0x0000e204      bd59e3         lda 0xe359,x
0x0000e207      2040e0         jsr 0xe040
0x0000e20a      4c28e3         jmp 0xe328
0x0000e20d      bd59e3         lda 0xe359,x
0x0000e210      2040e0         jsr 0xe040
0x0000e213      bc6be3         ldy 0xe36b,x
0x0000e216      f01d           beq 0x00e235
0x0000e218      b9fce4         lda 0xe4fc,y
0x0000e21b      f003           beq 0x00e220
0x0000e21d      9d6ce3         sta 0xe36c,x
0x0000e220      b9fde4         lda 0xe4fd,y
0x0000e223      c9ff           cmp #0xff
0x0000e225      c8             iny
0x0000e226      98             tya
0x0000e227      9004           bcc 0x00e22d
0x0000e229      18             clc
0x0000e22a      b93ce5         lda 0xe53c,y
0x0000e22d      9d6be3         sta 0xe36b,x
0x0000e230      b93be5         lda 0xe53b,y
0x0000e233      d019           bne 0x00e24e
0x0000e235      bd80e3         lda 0xe380,x
0x0000e238      f02b           beq 0x00e265
0x0000e23a      bc68e3         ldy 0xe368,x
0x0000e23d      b916e0         lda 0xe016,y
0x0000e240      8d4ce2         sta 0xe24c
0x0000e243      bc69e3         ldy 0xe369,x
0x0000e246      b9c8e5         lda 0xe5c8,y
0x0000e249      85fe           sta 0xfe
0x0000e24b      4c61e0         jmp 0xe061
0x0000e24e      1005           bpl 0x00e255
0x0000e250      7d81e3         adc 0xe381,x
0x0000e253      297f           and #0x7f
0x0000e255      a8             tay
0x0000e256      b9b5e3         lda 0xe3b5,y
0x0000e259      9d95e3         sta 0xe395,x
0x0000e25c      b90ee4         lda 0xe40e,y
0x0000e25f      9d96e3         sta 0xe396,x
0x0000e262      bd80e3         lda 0xe380,x
0x0000e265      ddace3         cmp 0xe3ac,x
0x0000e268      f043           beq 0x00e2ad
0x0000e26a      bc6de3         ldy 0xe36d,x
0x0000e26d      f03b           beq 0x00e2aa
0x0000e26f      1d56e3         ora 0xe356,x
0x0000e272      f036           beq 0x00e2aa
0x0000e274      bd6ee3         lda 0xe36e,x
0x0000e277      d011           bne 0x00e28a
0x0000e279      b97ce5         lda 0xe57c,y
0x0000e27c      1009           bpl 0x00e287
0x0000e27e      b989e5         lda 0xe589,y
0x0000e281      9d97e3         sta 0xe397,x
0x0000e284      4c9be2         jmp 0xe29b
0x0000e287      9d6ee3         sta 0xe36e,x
0x0000e28a      bd97e3         lda 0xe397,x
0x0000e28d      18             clc
0x0000e28e      7989e5         adc 0xe589,y
0x0000e291      6900           adc #0x00
0x0000e293      9d97e3         sta 0xe397,x
0x0000e296      de6ee3         dec 0xe36e,x
0x0000e299      d00f           bne 0x00e2aa
0x0000e29b      b97de5         lda 0xe57d,y
0x0000e29e      c9ff           cmp #0xff
0x0000e2a0      c8             iny
0x0000e2a1      98             tya
0x0000e2a2      9003           bcc 0x00e2a7
0x0000e2a4      b989e5         lda 0xe589,y
0x0000e2a7      9d6de3         sta 0xe36d,x
0x0000e2aa      4c28e3         jmp 0xe328
0x0000e2ad      bc7ee3         ldy 0xe37e,x
0x0000e2b0      b974e4         lda 0xe474,y
0x0000e2b3      85fe           sta 0xfe
0x0000e2b5      b992e4         lda 0xe492,y
0x0000e2b8      85ff           sta 0xff
0x0000e2ba      bc56e3         ldy 0xe356,x
0x0000e2bd      b1fe           lda (0xfe),y
0x0000e2bf      c940           cmp #0x40
0x0000e2c1      9018           bcc 0x00e2db
0x0000e2c3      c960           cmp #0x60
0x0000e2c5      901e           bcc 0x00e2e5
0x0000e2c7      c9c0           cmp #0xc0
0x0000e2c9      902e           bcc 0x00e2f9
0x0000e2cb      bd57e3         lda 0xe357,x
0x0000e2ce      d002           bne 0x00e2d2
0x0000e2d0      b1fe           lda (0xfe),y
0x0000e2d2      6900           adc #0x00
0x0000e2d4      9d57e3         sta 0xe357,x
0x0000e2d7      f046           beq 0x00e31f
0x0000e2d9      d04d           bne 0x00e328
0x0000e2db      9d82e3         sta 0xe382,x
0x0000e2de      c8             iny
0x0000e2df      b1fe           lda (0xfe),y
0x0000e2e1      c960           cmp #0x60
0x0000e2e3      b014           bcs 0x00e2f9
0x0000e2e5      c950           cmp #0x50
0x0000e2e7      290f           and #0x0f
0x0000e2e9      9d58e3         sta 0xe358,x
0x0000e2ec      f006           beq 0x00e2f4
0x0000e2ee      c8             iny
0x0000e2ef      b1fe           lda (0xfe),y
0x0000e2f1      9d59e3         sta 0xe359,x
0x0000e2f4      b029           bcs 0x00e31f
0x0000e2f6      c8             iny
0x0000e2f7      b1fe           lda (0xfe),y
0x0000e2f9      c9bd           cmp #0xbd
0x0000e2fb      9006           bcc 0x00e303
0x0000e2fd      f020           beq 0x00e31f
0x0000e2ff      09f0           ora #0xf0
0x0000e301      d019           bne 0x00e31c
0x0000e303      7d54e3         adc 0xe354,x
0x0000e306      9d6ae3         sta 0xe36a,x
0x0000e309      bd58e3         lda 0xe358,x
0x0000e30c      c903           cmp #0x03
0x0000e30e      f00f           beq 0x00e31f
0x0000e310      a900           lda #0x00
0x0000e312      9da8e3         sta 0xe3a8,x
0x0000e315      a90f           lda #0x0f
0x0000e317      9da7e3         sta 0xe3a7,x
0x0000e31a      a9fe           lda #0xfe
0x0000e31c      9d83e3         sta 0xe383,x
0x0000e31f      c8             iny
0x0000e320      b1fe           lda (0xfe),y
0x0000e322      f001           beq 0x00e325
0x0000e324      98             tya
0x0000e325      9d56e3         sta 0xe356,x
0x0000e328      bd97e3         lda 0xe397,x
0x0000e32b      9d02d4         sta 0xd402,x
0x0000e32e      9d03d4         sta 0xd403,x
0x0000e331      bda8e3         lda 0xe3a8,x
0x0000e334      9d06d4         sta 0xd406,x
0x0000e337      bda7e3         lda 0xe3a7,x
0x0000e33a      9d05d4         sta 0xd405,x
0x0000e33d      bd95e3         lda 0xe395,x
0x0000e340      9d00d4         sta 0xd400,x
0x0000e343      bd96e3         lda 0xe396,x
0x0000e346      9d01d4         sta 0xd401,x
0x0000e349      bd6ce3         lda 0xe36c,x
0x0000e34c      3d83e3         and 0xe383,x
0x0000e34f      9d04d4         sta 0xd404,x
0x0000e352      60             rts
*/

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct DummyPlugin {
	int dummy;

} DummyPlugin;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void* create_instance(ServiceFunc* serviceFunc) {
	(void)serviceFunc;

	DummyPlugin* plugin = (DummyPlugin*)malloc(sizeof(DummyPlugin));
	memset(plugin, 0, sizeof(DummyPlugin));

    return plugin;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void destroy_instance(void* user_data) {
	free(user_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void write_register(PDWriter* writer, const char* name, uint8_t size, uint16_t reg, uint8_t read_only) {
    PDWrite_array_entry_begin(writer);
    PDWrite_string(writer, "name", name);
    PDWrite_u8(writer, "size", size);

    if (read_only) {
        PDWrite_u8(writer, "read_only", 1);
	}

    if (size == 2) {
        PDWrite_u16(writer, "register", reg);
	} else {
        PDWrite_u8(writer, "register", (uint8_t)reg);
	}

    PDWrite_entry_end(writer);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void send_6502_registers(PDWriter* writer) {
	PDWrite_event_begin(writer, PDEventType_SetRegisters);
	PDWrite_array_begin(writer, "registers");

	write_register(writer, "pc", 2, 0x4444, 1);
	write_register(writer, "sp", 1, 1, 0);
	write_register(writer, "a", 1, 2, 0);
	write_register(writer, "x", 1, 3, 0);
	write_register(writer, "y", 1, 4, 0);

	PDWrite_array_end(writer);
	PDWrite_event_end(writer);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void on_menu(PDReader* reader) {
    uint32_t menuId;

    PDRead_find_u32(reader, &menuId, "menu_id", 0);

    switch (menuId) {
        case 1:
        {
        	printf("id 1 pressed!\n");
            break;
        }

        case 2:
        {
        	printf("id 2 pressed!\n");
            break;
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static PDDebugState update(void* user_data,
						   PDAction action,
						   PDReader* reader,
						   PDWriter* writer) {
    uint32_t event;

	(void)user_data;
    (void)action;
    (void)reader;
    (void)writer;

    while ((event = PDRead_get_event(reader))) {
        switch (event) {
            case PDEventType_MenuEvent:
			{
                on_menu(reader);
                break;
            }
		}
	}

	send_6502_registers(writer);

    // printf("Update backend\n");

    return PDDebugState_NoTarget;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static PDMenuHandle register_menu(void* user_data, PDMenuFuncs* menu_funcs) {
	(void)user_data;

	PDMenuHandle menu = PDMenu_create_menu(menu_funcs, "Dummy Backend Menu");

	PDMenu_add_menu_item(menu_funcs, menu, "Id 1", 1, 0, 0);
	PDMenu_add_menu_item(menu_funcs, menu, "Id 2", 2, 0, 0);

	return menu;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static PDBackendPlugin plugin =
{
    "Dummy Backend",
    create_instance,
    destroy_instance,
	register_menu,
    update,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

PD_EXPORT void InitPlugin(RegisterPlugin* registerPlugin, void* private_data) {
    registerPlugin(PD_BACKEND_API_VERSION, &plugin, private_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

