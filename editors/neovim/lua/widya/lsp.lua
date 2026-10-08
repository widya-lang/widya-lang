-- Neovim Native LSP Setup untuk Widya-Lang
-- Menggunakan biner LSP bawaan compiler: 'widya lsp'

local M = {}

function M.setup(opts)
  opts = opts or {}
  local cmd = opts.cmd or { "widya", "lsp" }

  vim.api.nvim_create_autocmd("FileType", {
    pattern = "widya",
    callback = function(args)
      local client_id = vim.lsp.start({
        name = "widya-lsp",
        cmd = cmd,
        root_dir = vim.fs.root(args.buf, { "widya.toml", ".git" }) or vim.fn.getcwd(),
        settings = {},
      })
      if client_id then
        vim.lsp.buf_attach_client(args.buf, client_id)
      end
    end,
  })
end

return M
