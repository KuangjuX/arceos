(function() {var implementors = {
"axerrno":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"axerrno/enum.AxError.html\" title=\"enum axerrno::AxError\">AxError</a>&gt; for <a class=\"enum\" href=\"axerrno/enum.LinuxError.html\" title=\"enum axerrno::LinuxError\">LinuxError</a>"]],
"axfs":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"axfs/fops/struct.OpenOptions.html\" title=\"struct axfs::fops::OpenOptions\">OpenOptions</a>&gt; for <a class=\"struct\" href=\"capability/struct.Cap.html\" title=\"struct capability::Cap\">Cap</a>"]],
"axhal":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"axhal/mem/struct.MemRegionFlags.html\" title=\"struct axhal::mem::MemRegionFlags\">MemRegionFlags</a>&gt; for <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>"]],
"capability":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"capability/struct.CapError.html\" title=\"struct capability::CapError\">CapError</a>&gt; for <a class=\"enum\" href=\"axerrno/enum.AxError.html\" title=\"enum axerrno::AxError\">AxError</a>"]],
"memory_addr":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>"]],
"page_table":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>"]],
"page_table_entry":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"page_table_entry/x86_64/struct.PTF.html\" title=\"struct page_table_entry::x86_64::PTF\">PageTableFlags</a>&gt; for <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"page_table_entry/aarch64/struct.DescriptorAttr.html\" title=\"struct page_table_entry::aarch64::DescriptorAttr\">DescriptorAttr</a>&gt; for <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>&gt; for <a class=\"struct\" href=\"page_table_entry/riscv/struct.PTEFlags.html\" title=\"struct page_table_entry::riscv::PTEFlags\">PTEFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>&gt; for <a class=\"struct\" href=\"page_table_entry/aarch64/struct.DescriptorAttr.html\" title=\"struct page_table_entry::aarch64::DescriptorAttr\">DescriptorAttr</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"page_table_entry/riscv/struct.PTEFlags.html\" title=\"struct page_table_entry::riscv::PTEFlags\">PTEFlags</a>&gt; for <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>&gt; for <a class=\"struct\" href=\"page_table_entry/x86_64/struct.PTF.html\" title=\"struct page_table_entry::x86_64::PTF\">PTF</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()