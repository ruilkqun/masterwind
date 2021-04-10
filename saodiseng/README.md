## 下载镜像
```
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  pull docker:ruilkyu/nginx:latest

Image is up to date for ruilkyu/nginx@sha256:bd877619f4ab21d0d2a26c622c0c51935d4da763203d83f542e39a4720d09bdc



[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  pull docker:nginx:latest
Image is up to date for library/nginx@sha256:7ce4f91ef623b9672ec12302c4a710629cd542617c1ebc616a48d06e2a84656a



[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock pull --creds=admin:saodiseng harbor:192.168.1.118:8899/saodiseng/nginx:latest
Image is up to date for 192.168.1.118:8899saodiseng/nginx@sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe
```

## 查看镜像列表
```
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  images docker:nginx:latest
IMAGE               TAG                 IMAGE ID            SIZE
<none>              <none>                                  0B
```
