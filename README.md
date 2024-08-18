# Gentoo 日常运维工具

## 安装方法

### 创建 admin 用户

```bash
useradd admin -g wheel
```

### 修改 profile

新建文件 `/etc/profile.d/admin.sh` ：

```bash
[ "$USER" == "admin" ] && shopt -q login_shell && exec /opt/rust-tui-demo
```
