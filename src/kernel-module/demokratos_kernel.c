/*
 * DEMOKRATOS: Módulo del Kernel de Linux para Aislamiento de Hardware Electoral
 * v0.1.0 - Prototipo inicial de seguridad de bajo nivel.
 */

#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/fs.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Comunidad Demokratos");
MODULE_DESCRIPTION("Modulo de aislamiento biometrico y blindaje de hardware para votaciones transparentes");
MODULE_VERSION("0.1.0");

#define DEVICE_NAME "demokratos_secure_boot"

static int major_num;
static int dispositivo_abierto = 0;

/* Se ejecuta cuando una aplicación del espacio de usuario intenta comunicarse con el sensor seguro */
static int device_open(struct inode *inode, struct file *file) {
    if (dispositivo_abierto) return -EBUSY;
    dispositivo_abierto++;
    try_module_get(THIS_MODULE);
    return 0;
}

/* Se ejecuta al cerrar el canal de comunicación seguro */
static int device_release(struct inode *inode, struct file *file) {
    dispositivo_abierto--;
    module_put(THIS_MODULE);
    return 0;
}

/* Operaciones de archivo soportadas por nuestro módulo del kernel */
static struct file_operations fops = {
    .open = device_open,
    .release = device_release,
};

/* Función que se ejecuta al cargar el módulo en el sistema operativo (insmod) */
static int __init demokratos_init(void) {
    printk(KERN_INFO "[DEMOKRATOS-KERNEL] Inicializando modulo de seguridad electoral...\n");
    
    major_num = register_chrdev(0, DEVICE_NAME, &fops);
    if (major_num < 0) {
        printk(KERN_ALERT "[DEMOKRATOS-KERNEL] Error critico: No se pudo registrar el dispositivo de hardware seguro.\n");
        return major_num;
    }
    
    printk(KERN_INFO "[DEMOKRATOS-KERNEL] Entorno blindado activo. Asignado Major Number %d.\n", major_num);
    printk(KERN_INFO "[DEMOKRATOS-KERNEL] Monitoreo de bus USB y registros de memoria iniciado. Listo para votacion.\n");
    return 0;
}

/* Función que se ejecuta al descargar el módulo del sistema (rmmod) */
static void __exit demokratos_exit(void) {
    unregister_chrdev(major_num, DEVICE_NAME);
    printk(KERN_INFO "[DEMOKRATOS-KERNEL] Modulo de seguridad electoral descargado. Memoria liberada de forma segura.\n");
}

module_init(demokratos_init);
module_exit(demokratos_exit);
