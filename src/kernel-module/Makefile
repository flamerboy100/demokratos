# Makefile para compilar el módulo del kernel de DEMOKRATOS
# Indica al sistema de compilación de Linux que genere el objeto demokratos_kernel.o
obj-m += demokratos_kernel.o

# Detecta automáticamente la ruta de las cabeceras del kernel actual del sistema
KDIR /lib/modules/$(shell uname -r)/build

all:
	$(MAKE) -C $(KDIR) M=$(PWD) modules

clean:
	$(MAKE) -C $(KDIR) M=$(PWD) clean
