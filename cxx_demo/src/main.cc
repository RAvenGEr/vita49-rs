// SPDX-FileCopyrightText: 2025 The vita49-rs Authors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx_demo/include/my_vrt.h"
#include "cxx_demo/src/main.rs.h"
#include <functional>
#include <string>
#include <vector>
#include <iostream>
#include <fstream>

int main(void) {
    std::ifstream file("../vita49/tests/spectral_data_packet.vrt", std::ios::binary);
    std::vector<uint8_t> input(std::istreambuf_iterator<char>(file), {});
    rust::Slice<const uint8_t> slice{input.data(), input.size()};
    MySignalData ret = parse_vita49(slice);
    printf("[C++] Got data packet with stream ID: 0x%X\n", ret.stream_id);
}
