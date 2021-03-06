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
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  images
IMAGE               TAG                 IMAGE ID            SIZE
/saodiseng/nginx    latest              6084105296a95       137MB
library/nginx       latest              519e12e2a84a9       137MB

[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  images docker:nginx:latest
IMAGE               TAG                 IMAGE ID            SIZE
library/nginx       latest              519e12e2a84a9       137MB
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  images harbor:/saodiseng/nginx:latest
IMAGE               TAG                 IMAGE ID            SIZE
/saodiseng/nginx    latest              6084105296a95       137MB

[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock
 images --digests sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe
IMAGE               TAG                 DIGEST              IMAGE ID            SIZE
/saodiseng/nginx    latest              <none>              6084105296a95       137MB
```

## 删除镜像
```
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  rmi 519e12e2a84a9
Deleted: library/nginx:latest
```

## 查看镜像信息
```
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  inspecti 6084105296a95
{
  "status": {
    "id": "sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe",
    "repoTags": [
      "/saodiseng/nginx:latest"
    ],
    "repoDigests": [],
    "size": "137318912",
    "uid": {
      "value": "1000"
    },
    "username": "root",
    "spec": {
      "image": "sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe",
      "annotations": {}
    }
  }
}
```

## 查看镜像文件系统信息
```
[root@localhost ~]# crictl --image-endpoint unix:///var/run/saodiseng.sock  imagefsinfo
{
  "status": {
    "timestamp": "1618385746189724737",
    "fsId": {
      "mountpoint": "/"
    },
    "usedBytes": {
      "value": "547010913"
    },
    "inodesUsed": {
      "value": "71199008"
    }
  }
}
```
