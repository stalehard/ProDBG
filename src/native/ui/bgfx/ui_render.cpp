#include <bgfx/bgfx.h>
//#include "core/log.h"
//#include "core/file.h"
//#include "core/core.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#define sizeof_array(t) (sizeof(t) / sizeof(t[0]))

#if defined(__clang__) || defined(__gcc__)
#define PD_NO_RETURN __attribute__((noreturn))
#else
#define PD_NO_RETURN
#endif

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void* File_loadToMemory(const char* filename, size_t* size, size_t padAllocSize) {
    FILE* f = fopen(filename, "rb");
    void* data = 0;
    size_t s = 0, t = 0;

    *size = 0;

    if (!f)
        return 0;

    // TODO: Use fstat here?

    fseek(f, 0, SEEK_END);
    long ts = ftell(f);

    if (ts < 0)
        goto end;

    s = (size_t)ts;

    data = malloc(s + padAllocSize);

    if (!data)
        goto end;

    fseek(f, 0, SEEK_SET);

    t = fread(data, s, 1, f);
    (void)t;

    *size = s;

    end:

    fclose(f);

    return data;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bgfx::UniformHandle s_tex;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ProgramAttribs {
    bgfx::Attrib::Enum attrib;
    uint8_t num;
    bgfx::AttribType::Enum type;
    bool norm;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ProgramInfo {
    ProgramAttribs* attribs;
    const char* vsName;
    const char* fsName;
    bgfx::VertexDecl vertexDecl;
    bgfx::ProgramHandle handle;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const bgfx::Memory* loadShader(const char* filename) {
    size_t size;
    uint8_t* data = (uint8_t*)File_loadToMemory(filename, &size, 1);

    if (!data) {
        printf("Unable to load shader %s\n", filename);
        return 0;
    }
    
    const bgfx::Memory *mem = bgfx::makeRef(data, (uint32_t)size, NULL, NULL);

    // terminate strings
    data[size] = 0;

    return mem;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

bgfx::ProgramHandle loadProgram(const char* vsName, const char* fsName) {
    bgfx::ProgramHandle ph = { bgfx::invalidHandle };

    const bgfx::Memory* vsShader = loadShader(vsName);
    const bgfx::Memory* fsShader = loadShader(fsName);

    if (!vsShader)
        return ph;

    if (!fsShader)
        return ph;

    bgfx::ShaderHandle vsHandle = bgfx::createShader(vsShader);
    bgfx::ShaderHandle fsHandle = bgfx::createShader(fsShader);

    if (!isValid(vsHandle)) {
        printf("Unable to load vsShader %s\n", vsName);
        return ph;
    }

    if (!isValid(fsHandle)) {
        printf("Unable to load fsShader %s\n", fsName);
        return ph;
    }

    ph = bgfx::createProgram(vsHandle, fsHandle, true);

    if (!isValid(ph))
        printf("Unable to create shader program for %s %s\n", vsName, fsName);

    return ph;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static ProgramAttribs posTexColorAttribs[] =
{
    { bgfx::Attrib::Position, 2, bgfx::AttribType::Float, false },
    { bgfx::Attrib::TexCoord0, 2, bgfx::AttribType::Float, false },
    { bgfx::Attrib::Color0, 4, bgfx::AttribType::Uint8, true },
    { bgfx::Attrib::Count, 0, bgfx::AttribType::Uint8, false },
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static ProgramAttribs posColorAttribs[] =
{
    { bgfx::Attrib::Position, 2, bgfx::AttribType::Float, false },
    { bgfx::Attrib::Color0, 4, bgfx::AttribType::Uint8, true },
    { bgfx::Attrib::Count, 0, bgfx::AttribType::Uint8, false },
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static ProgramInfo s_programs[] =
{
    {
        (ProgramAttribs*)&posColorAttribs,
        OBJECT_DIR "/_generated/data/shaders/ui_pos_color/vs_pos_color.vs",
        OBJECT_DIR "/_generated/data/shaders/ui_pos_color/fs_pos_color.fs",
    },

    {
        (ProgramAttribs*)&posTexColorAttribs,
        OBJECT_DIR "/_generated/data/shaders/ui_pos_tex_r_color/vs_pos_tex_r_color.vs",
        OBJECT_DIR "/_generated/data/shaders/ui_pos_tex_r_color/fs_pos_tex_r_color.fs",
    },

    {
        (ProgramAttribs*)&posTexColorAttribs,
        OBJECT_DIR "/_generated/data/shaders/imgui/vs_imgui.vs",
        OBJECT_DIR "/_generated/data/shaders/imgui/fs_imgui.fs",
    },
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum {
    Program_PosColor,
    Program_PosTexRColor,
    Program_PosTexColor,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

bool UIRender_init() {
    s_tex = bgfx::createUniform("s_tex", bgfx::UniformType::Int1);

    for (int i = 0; i < (int)sizeof_array(s_programs); ++i) {
        ProgramInfo* program = &s_programs[i];

        program->handle = loadProgram(program->vsName, program->fsName);

        if (!isValid(program->handle))
            return false;

        const ProgramAttribs* attribs = program->attribs;
        bgfx::VertexDecl& decl = program->vertexDecl;

        decl.begin();

        while (attribs->attrib != bgfx::Attrib::Count) {
            decl = decl.add(attribs->attrib, attribs->num, attribs->type, attribs->norm);
            attribs++;
        }

        decl.end();
    }

    return true;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void UIRender_allocPosTexColorTb(bgfx::TransientVertexBuffer* buffer, uint32_t count) {
    assert(bgfx::checkAvailTransientVertexBuffer(count, s_programs[Program_PosTexColor].vertexDecl));
    bgfx::allocTransientVertexBuffer(buffer, count, s_programs[Program_PosTexColor].vertexDecl);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void UIRender_allocPosColorTb(bgfx::TransientVertexBuffer* buffer, uint32_t count) {
    assert(bgfx::checkAvailTransientVertexBuffer(count, s_programs[Program_PosColor].vertexDecl));
    bgfx::allocTransientVertexBuffer(buffer, count, s_programs[Program_PosColor].vertexDecl);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void UIRender_posTexColor(bgfx::TransientVertexBuffer* vertexBuffer, uint32_t offset, uint32_t count, bgfx::TextureHandle texHandle) {
    bgfx::setTexture(0, s_tex, texHandle);
    bgfx::setVertexBuffer(vertexBuffer, offset, count);
    //NOTE(marco): the program handle is now part of bgfx::submit
    //bgfx::setProgram(s_programs[Program_PosTexColor].handle);
    bgfx::submit(0, s_programs[Program_PosTexColor].handle);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void UIRender_posIdxTexColor(bgfx::TransientVertexBuffer* vertexBuffer, bgfx::TransientIndexBuffer* indexBuffer, uint32_t vtxSize, uint32_t offset, uint32_t count, bgfx::TextureHandle texHandle) {
    bgfx::setTexture(0, s_tex, texHandle);
    bgfx::setVertexBuffer(vertexBuffer, 0, vtxSize);
    bgfx::setIndexBuffer(indexBuffer, offset, count);
    //NOTE(marco): the program handle is now part of bgfx::submit
    //bgfx::setProgram(s_programs[Program_PosTexColor].handle);
    bgfx::submit(0, s_programs[Program_PosTexColor].handle);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void UIRender_posTexRColor(bgfx::TransientVertexBuffer* vertexBuffer, uint32_t offset, uint32_t count, bgfx::TextureHandle texHandle) {
    bgfx::setTexture(0, s_tex, texHandle);
    bgfx::setVertexBuffer(vertexBuffer, offset, count);
    //NOTE(marco): the program handle is now part of bgfx::submit
    //bgfx::setProgram(s_programs[Program_PosTexRColor].handle);
    bgfx::submit(0, s_programs[Program_PosTexRColor].handle);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void UIRender_posColor(bgfx::TransientVertexBuffer* vertexBuffer, uint32_t offset, uint32_t count) {
    bgfx::setVertexBuffer(vertexBuffer, offset, count);
    //NOTE(marco): the program handle is now part of bgfx::submit
    //bgfx::setProgram(s_programs[Program_PosColor].handle);
    bgfx::submit(0, s_programs[Program_PosColor].handle);
}

bgfx::ProgramHandle UIRender_getProgramHandle(uint32_t programIndex) {
    return s_programs[2].handle;
}





