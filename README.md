# THIS PROJECT IS NOW OBSOLETE. SEE [outcrop](https://github.com/0x4c37373230/outcrop-final)
#### I decided to archive all my standalone bedrock reverse engineering and modding projects in order to combine all of them into a single, user friendly tool so others can use them as well. The MCBE IDA Class Generator and all other BDS tools will be continued and maintained there.

# MCBE IDA Class Generator

A relatively simple rust program to generate modern C++ compliant fully usable classes from IDA data corresponding to the Bedrock Dedicated Server, without breaking the Mojang EULA since they contain no Mojang code.

As simple and brainless as this code might seem, this took me a decent while to make and even though I could have just replicated most of the classes I needed to use while modding manually, I decided to follow [Zhuowei Zang's advice](https://twitter.com/zhuowei/status/1254266079532154880?lang=en)

## Usage

This "tool" is relatively simple to use and even though it was made mostly to be used with information extracted from the 1.6 Windows Bedrock Dedicated Server, it should work fine in later versions.

First things first, a file named `class.txt` must be created in the same directory as the executable. This file will be divided into 2 sections, `=====DECLA=====` and `=====INMEM=====`, having `=====ENDSF=====` at the end of the file. Between the first 2 'delimiters', you'll have to paste the struct declaration. You can easily get this by opening the IDA 'Local Types' window, right-clicking on a struct and going to the 'edit' section. Just copy-paste the contents. Then, between the last 2 you'll have to paste the class memory layout which you can easily get by opening the structs window and clicking on whichever struct you need (you can use Ctrl+F to find them in both windows), or double-clicking on the struct you want in the local types window. Just copy-paste the memory layout.

Once that's done you can just fire up a command prompt, run something along these lines `ida-mcbe-class-formatter > out.txt`, and you'll have your output class in `out.txt`. This project contains an example `class.txt` which produces the following output

```cpp
const class Block
{
public:
	auto getData() -> const unsigned __int8 {
		return *reinterpret_cast<const unsigned __int8>(reintepret_cast<VA>(this) + 0x8); 
	}
	auto getLegacyBlock() -> const BlockLegacy *{
		return reinterpret_cast<const BlockLegacy *>(reintepret_cast<VA>(this) + 0x10); 
	}
	auto getSerializationId() -> CompoundTag {
		return *reinterpret_cast<CompoundTag>(reintepret_cast<VA>(this) + 0x18); 
	}
	auto getRuntimeId() -> unsigned int {
		return *reinterpret_cast<unsigned int>(reintepret_cast<VA>(this) + 0x30); 
	}
	auto setData(const unsigned __int8  param_1) -> void {
		*reinterpret_cast<const unsigned __int8>(reintepret_cast<VA>(this) + 0x8) = param_1;
	}
	auto setLegacyBlock(const BlockLegacy * param_1) -> void {
		reinterpret_cast<const BlockLegacy *>(reintepret_cast<VA>(this) + 0x10) = param_1;
	}
	auto setSerializationId(CompoundTag  param_1) -> void {
		*reinterpret_cast<CompoundTag>(reintepret_cast<VA>(this) + 0x18) = param_1;
	}
	auto setRuntimeId(unsigned int  param_1) -> void {
		*reinterpret_cast<unsigned int>(reintepret_cast<VA>(this) + 0x30) = param_1;
	}
};

```
