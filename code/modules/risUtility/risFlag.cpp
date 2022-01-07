#include "pch.h"
#include "risFlag.h"

namespace risUtility
{
	struct risFlag::Impl
	{
		constexpr static U8 flag_count = 64;
		FlagCollection* flags = nullptr;

		Impl() : flags(new FlagCollection(0)) { }
		~Impl() { delete flags; }

		constexpr static FlagCollection mask(U8 flag)
		{
			return static_cast<FlagCollection>(1) << flag;
		}
	};

	risFlag::risFlag() : pImpl(new Impl()) { }
	risFlag::~risFlag() { delete pImpl; }

	void risFlag::apply(FlagCollection flags) const
	{
		*pImpl->flags = flags;
	}

	risFlag::FlagCollection risFlag::retrieve() const
	{
		return *pImpl->flags;
	}

	bool risFlag::get(U8 flag) const
	{
		if (flag >= Impl::flag_count)
			return false;

		const auto value = *pImpl->flags;
		const auto mask = Impl::mask(flag);

		return (value & mask) != 0;
	}

	void risFlag::set(U8 flag, bool value) const
	{
		if (flag >= Impl::flag_count)
			return;

		const auto mask = Impl::mask(flag);

		if (value)
			*pImpl->flags |= mask;
		else
			*pImpl->flags &= ~mask;
	}

	void risFlag::toggle(U8 flag) const
	{
		if (flag >= Impl::flag_count)
			return;

		const auto mask = Impl::mask(flag);
		*pImpl->flags ^= mask;
	}

	std::string risFlag::toString() const
	{
		constexpr U8 groupBy = 8;

		std::string result;
		for (U8 i = 0; i < Impl::flag_count; ++i)
		{
			if (i != 0 && i % groupBy == 0)
				result.append(" ");

			result.append(get(i) ? "1" : "0");
		}

		return result;
	}
}
