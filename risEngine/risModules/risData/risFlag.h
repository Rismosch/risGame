#pragma once
#include "risPrimitives.h"

namespace risEngine
{
	class risFlag
	{
	public:
		typedef U64 FlagCollection;
		typedef U8 Flag;
		
		void apply(FlagCollection flags);
		FlagCollection retrieve() const;
		
		bool get(Flag flag) const;
		void set(Flag flag, bool value);
		void toggle(Flag flag);

#if defined _DEBUG
		const char* to_string();
#else
		const char* to_string() = delete;
#endif

	private:
		constexpr static auto flag_count_ = sizeof(FlagCollection) * 8;
		FlagCollection flags_ = 0;
	};
}
