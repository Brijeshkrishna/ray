// // #include <iostream>
// // #include <cstring>
// // #include <sys/socket.h>
// // #include <sys/ioctl.h>
// // #include <linux/wireless.h>
// // #include <vector>
// // #include <string>
// // #include <bitset>
// // #include <arpa/inet.h>
// // #include <vector>

// // using namespace std;

// // void list_of_devices( int sock){

// //     struct ifconf ifc;
// //     struct ifreq  ifr[10];

// //     ifc.ifc_len = sizeof(ifr);
// //     ifc.ifc_ifcu.ifcu_req = ifr;

// //     if ( ioctl(sock, SIOCGIFCONF, &ifc) <0) {
// //         perror("ioctl");
// //     }

// //     cout << ifr[1].ifr_ifrn.ifrn_name;

// // }

// // int main(){

// //     int sockfd = socket(AF_INET, SOCK_DGRAM, 0);

// //     struct ifreq ifr;

// //     // struct ifconf ifc;
// //     // ifr.ifr_ifru.ifru_ivalue;
// //       strncpy(ifr.ifr_ifrn.ifrn_name, "wlp5s0f4u2", IFNAMSIZ - 1);

// //     if (ioctl(sockfd, SIOCGIWMODE, &ifr) == 0) {
// //         printf("yes");
// //     }SIOCGIFFLAGS
// //     else{
// //         printf("no");
// //     }
// //     __errno_location();

// //     // ioctl(sockfd, SIOCGIFNAME, &ifr);

// //     // ioctl(sockfd, SIOCGIFFLAGS, &ifr);
// //     // printf("%d\n",ifr.ifr_ifru.ifru_settings.type);

// //     // ioctl(sockfd, SIOCGIFDSTADDR, &ifr);
// //     // printf("%d\n",ifr.ifr_ifru.ifru_dstaddr);

// //     // ioctl(sockfd, SIOCGIFNETMASK, &ifr);
// //     // printf("%d\n",ifr.ifr_ifru.ifru_netmask);

// //     // ioctl(sockfd, SIOCGIFADDR, &ifr);
// //     // printf("%d\n",ifr.ifr_ifru.ifru_addr);

// //     // ioctl(sockfd, SIOCGIFBRDADDR, &ifr);
// //     // printf("%d\n",ifr.ifr_ifru.ifru_broadaddr);

// //     // ioctl(sockfd, SIOCGIFMTU, &ifr);
// //     // printf("%d\n",ifr.ifr_ifru.ifru_mtu);

// //     // list_of_devices(sockfd);

// // }

// // // #include <stdio.h>
// // // #include <stdlib.h>
// // // #include <string.h>
// // // #include <unistd.h>
// // // #include <sys/ioctl.h>
// // // #include <net/if.h>
// // // #include <linux/wireless.h>

// // // int main() {
// // //     int sock;
// // //     struct iwreq wrq;
// // //     char ifname[] = "wlp3s0"; // Replace with the name of your wireless interface

// // //     // Open a socket for wireless ioctl operations
// // //     sock = socket(AF_INET, SOCK_DGRAM, 0);
// // //     if (sock == -1) {
// // //         perror("socket");
// // //         exit(1);
// // //     }

// // //     // Set the interface name in the request
// // //     strncpy(wrq.ifr_ifrn.ifrn_name, ifname, IFNAMSIZ);

// // //     char buff[1000] ;

// // //     // Perform the ioctl operation to retrieve the ESSID
// // //     wrq.u.essid.length = sizeof(buff); // Ensure length is initialized to 0
// // //     wrq.u.essid.pointer = buff; // Initialize pointer to NULL

// // //     if (ioctl(sock, SIOCGIWSCAN, &wrq) == -1) {
// // //         perror("ioctl SIOCGIWESSID");
// // //         close(sock);
// // //         exit(1);
// // //     }

// // //     // Print the ESSID if available
// // //     if (wrq.u.essid.length > 0) {
// // //         printf("ESSID: %s\n", wrq.u.essid.pointer);
// // //     } else {
// // //         printf("ESSID is not available.\n");
// // //     }

// // //     // Close the socket
// // //     close(sock);

// // //     return 0;
// // // }

// // #include <iostream>
// // #include <cstring>
// // #include <sys/socket.h>
// // #include <sys/ioctl.h>
// // #include <net/if.h>
// // #include <netinet/in.h>
// // #include <arpa/inet.h>
// // #include <errno.h>

// // int main() {
// //     // Replace with the name of your network interface
// //     const char* interfaceName = "wlp3s0";
// //     // Replace with the new IP address you want to set
// //     const char* newIpAddress = "192.168.1.69";

// //     // Open a socket to interact with network interface configuration
// //     int socketFd = socket(AF_INET, SOCK_DGRAM, 0);
// //     if (socketFd == -1) {
// //         std::cerr << "Error opening socket: " << strerror(errno) << std::endl;
// //         return 1;
// //     }

// //     struct ifreq ifRequest;
// //     memset(&ifRequest, 0, sizeof(ifRequest));
// //     strncpy(ifRequest.ifr_name, interfaceName, IFNAMSIZ - 1);

// //     // Set the IP address in the ifreq structure
// //     struct sockaddr_in* sockaddr = (struct sockaddr_in*)&ifRequest.ifr_addr;
// //     sockaddr->sin_family = AF_INET;
// //     sockaddr->sin_port = 0;
// //     if (inet_pton(AF_INET, newIpAddress, &(sockaddr->sin_addr)) <= 0) {
// //         std::cerr << "Error converting IP address: " << strerror(errno) << std::endl;
// //         return 1;
// //     }

// //     // Use the SIOCSIFADDR ioctl command to set the IP address
// //     if (ioctl(socketFd, SIOCSIFADDR, &ifRequest) == -1) {
// //         std::cerr << "Error setting IP address: " << strerror(errno) << std::endl;
// //         return 1;
// //     }

// //     std::cout << "IP address set successfully." << std::endl;

// //     // Close the socket
// //     // close(socketFd);

// //     return 0;
// // }

// #include <iostream>
// #include <cstring>
// #include <sys/socket.h>
// #include <sys/ioctl.h>
// #include <net/if.h>
// #include <netinet/in.h>
// #include <arpa/inet.h>
// #include <errno.h>
// #include<linux/wireless.h>
// #include <stdio.h>
// #include <stdint.h>
// #include <stddef.h>
// #include <ctype.h>
// #include <dirent.h>
// #include <dlfcn.h>
// #include <elf.h>
// #include <fcntl.h>
// #include <getopt.h>
// #include <glob.h>
// #include <gnu/libc-version.h>
// #include <grp.h>
// #include <iconv.h>
// #include <ifaddrs.h>
// #include <langinfo.h>
// #include <libgen.h>
// #include <limits.h>
// #include <link.h>
// #include <locale.h>
// #include <malloc.h>
// #include <mntent.h>
// #include <mqueue.h>
// #include <net/ethernet.h>
// #include <net/if.h>
// #include <net/if_arp.h>
// #include <net/route.h>
// #include <netdb.h>
// #include <netinet/in.h>
// #include <netinet/ip.h>
// #include <netinet/tcp.h>
// #include <netinet/udp.h>
// #include <netpacket/packet.h>
// #include <poll.h>
// #include <pthread.h>
// #include <pty.h>
// #include <pwd.h>
// #include <regex.h>
// #include <resolv.h>
// #include <sched.h>
// #include <semaphore.h>
// #include <shadow.h>
// #include <signal.h>
// #include <spawn.h>
// #include <stddef.h>
// #include <stdint.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>
// #include <sys/epoll.h>
// #include <sys/eventfd.h>
// #include <sys/file.h>
// #include <sys/fsuid.h>
// #include <sys/inotify.h>
// #include <sys/ioctl.h>
// #include <sys/ipc.h>
// #include <sys/mman.h>
// #include <sys/mount.h>
// #include <sys/msg.h>
// #include <sys/personality.h>
// #include <sys/prctl.h>
// #include <sys/ptrace.h>
// #include <sys/quota.h>
// #include <sys/random.h>
// #include <sys/reboot.h>
// #include <sys/resource.h>
// #include <sys/sem.h>
// #include <sys/sendfile.h>
// #include <sys/shm.h>
// #include <sys/signalfd.h>
// #include <sys/socket.h>
// #include <sys/stat.h>
// #include <sys/statvfs.h>
// #include <sys/swap.h>
// #include <sys/syscall.h>
// #include <sys/time.h>
// #include <sys/timerfd.h>
// #include <sys/times.h>
// #include <sys/timex.h>
// #include <sys/types.h>
// #include <sys/uio.h>
// #include <sys/un.h>
// #include <sys/user.h>
// #include <sys/utsname.h>
// #include <sys/vfs.h>
// #include <sys/wait.h>
// #include <syslog.h>
// #include <termios.h>
// #include <time.h>
// #include <ucontext.h>
// #include <unistd.h>
// #include <utime.h>
// #include <utmp.h>
// #include <utmpx.h>
// #include <wchar.h>
// #include <errno.h>
// #include <sys/io.h>
// #include <sys/reg.h>
// #include <execinfo.h>
// #include <asm/mman.h>
// #include <linux/can.h>
// #include <linux/can/raw.h>
// #include <linux/can/j1939.h>
// #include <linux/dccp.h>
// #include <linux/errqueue.h>
// #include <linux/falloc.h>
// #include <linux/filter.h>
// #include <linux/fs.h>
// #include <linux/futex.h>
// #include <linux/genetlink.h>
// #include <linux/if.h>
// #include <linux/if_addr.h>
// #include <linux/if_alg.h>
// #include <linux/if_ether.h>
// #include <linux/if_tun.h>
// #include <linux/input.h>
// #include <linux/ipv6.h>
// #include <linux/kexec.h>
// #include <linux/keyctl.h>
// #include <linux/magic.h>
// #include <linux/memfd.h>
// #include <linux/membarrier.h>
// #include <linux/mempolicy.h>
// #include <linux/mman.h>
// #include <linux/module.h>
// #include <linux/mount.h>
// #include <linux/net_tstamp.h>
// #include <linux/netfilter/nfnetlink.h>
// #include <linux/netfilter/nfnetlink_log.h>
// #include <linux/netfilter/nfnetlink_queue.h>
// #include <linux/netfilter/nf_tables.h>
// #include <linux/netfilter_ipv4.h>
// #include <linux/netfilter_ipv6.h>
// #include <linux/netfilter_ipv6/ip6_tables.h>
// #include <linux/netlink.h>
// #include <linux/openat2.h>
// #include <linux/ptrace.h>
// #include <linux/quota.h>
// #include <linux/random.h>
// #include <linux/reboot.h>
// #include <linux/rtnetlink.h>
// #include <linux/sched.h>
// #include <linux/sctp.h>
// #include <linux/seccomp.h>
// #include <linux/sock_diag.h>
// #include <linux/sockios.h>
// #include <linux/tls.h>
// #include <linux/uinput.h>
// #include <linux/vm_sockets.h>
// #include <linux/wait.h>
// #include <sys/fanotify.h>
// #include <sys/auxv.h>
// #include <linux/close_range.h>
// #include <sys/xattr.h>
// #include <sys/sysinfo.h>
// #include <aio.h>

// int main() {
//     int socket_fd = socket(AF_INET, SOCK_DGRAM, 0);
//     if (socket_fd == -1) {
//         perror("Error opening socket");
//         return 1;
//     }
// // SIOCGIWSCAN;
//     struct ifconf ifconf;
//     char buffer[16384];

//     ifconf.ifc_buf = buffer;
//     ifconf.ifc_len = sizeof(buffer);

//     if (ioctl(socket_fd, SIOCGIFCONF, &ifconf) == -1) {
//         perror("Error getting interface list");
//         return 1;
//     }

//     struct ifreq* ifr = ifconf.ifc_req;
//     int num_interfaces = ifconf.ifc_len / sizeof(struct ifreq);

//     for (int i = 0; i < num_interfaces; i++) {
//         struct sockaddr_in* addr = (struct sockaddr_in*)&ifr[i].ifr_addr;
//         char ip_address[INET_ADDRSTRLEN];
//         inet_ntop(AF_INET, &addr->sin_addr, ip_address, sizeof(ip_address));

//         std::cout << "Interface: " << ifr[i].ifr_name << " IP: " << ip_address << std::endl;
//     }

//     return 0;
// }

// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>
// #include <unistd.h>
// #include <sys/ioctl.h>
// #include <netinet/in.h>
// #include <net/if.h>
// #include <linux/wireless.h>

// int main()
// {

//     char ifname[] = "wlp3s0"; // Replace with the name of your wireless interface
//     int sock = socket(AF_INET, SOCK_DGRAM, 0);
//     struct iwreq iw;

//     memset(&iw, 0, sizeof(iw));

//     // set interface to wlp3s0
//     strncpy(iw.ifr_name, ifname, IFNAMSIZ - 1); // IFNAMSIZ = 16 length of interface

//     //    // sys call
//     //    struct iw_range range;
//     //
//     //    iw.u.data.pointer = (caddr_t) &range;
//     //    iw.u.data.length = sizeof(range);
//     //    iw.u.data.flags = 0;

//     struct ifreq ifr;
//     strncpy(ifr.ifr_name, "wlp3s0", IFNAMSIZ); // IFNAMSIZ = 16 length of interface

//     unsigned char *buffer = (unsigned char *) malloc(IW_SCAN_MAX_DATA * 10);
//     //    iw.u.essid.pointer = (char*)malloc(32+1);
//     //    iw.u.essid.length = 32;
//     iw.u.data.pointer = buffer;
//     iw.u.data.flags = 0;

//     iw.u.data.length = IW_SCAN_MAX_DATA * 10;
//     // int a = iw_get_ext(sockfd,"wlp3s0",SIOCGIWSCAN,&iw);
//     // ioctl(sockfd,  SIOCGIWSCAN  , &iw)
//     while (1)
//     {
//         if (ioctl(sock, SIOCGIWSCAN, &iw) < 0)
//         {
//             perror("ioctl");
//         }
//         else
//         {
//             break;
//         }
//     }

//     // for (int i = 0 ; i <IW_SCAN_MAX_DATA;i++){
//     // }
//     //     printf("%lu",  sizeof(iw.u.data.pointer));

//     //    struct iw_statistics *stats = (struct iw_statistics *) & iw.u.data;
//     //
//     //    printf("Stats:\n");
//     //    printf("\tReceived packets: %d\n", stats->qual.level);
//     //    printf("\tDropped packets: %d\n", stats->discard.retries);
//     //    printf("\tRetransmitted packets: %d\n", stats->discard.misc);
//     //
//     //    printf("Interface flags for %d:\n",  ifr.ifr_ifru.ifru_flags);

//     if (ioctl(sock, SIOCGIWAP, &iw) == -1)
//     {
//         perror("IOCTL SIOCGIWAP Failed,error");
//         exit(2);
//     }
//     else
//     {
//         printf("IOCTL SIOCGIWAP Successfull\n");
//     }
//     for (int i = 0; i < 6; i++)
//     {
//         unsigned char *APaddr = (unsigned char *)iw.u.freq.i;
//         printf("%s\n",APaddr);
//         printf("%02x", (int)APaddr[i]); // mac[i] means 1 byte ,i.e. 8 bits.
//         if (i != 5)
//             printf("%c", ':');
//         else
//             printf("\n");
//     }
//     close(sock);

//     return 0;
// }

#include <stdio.h>
#include <time.h>
#include <iwlib.h>


int main(void)
{
  wireless_scan_head head;
  wireless_scan *result;
  iw_range range;
  int sock;
  const char name[] = "wlp3s0";

  sock = socket(AF_INET, SOCK_DGRAM, 0);

  if (iw_get_range_info(sock, name, &range) < 0)
  {
    printf("Error during iw_get_range_info. Aborting.\n");
    exit(2);
  }

  printf("%d",iw_process_scan(sock, "wlp3s0", range.we_version_compiled, &head));
  /* Perform the scan */
  if (iw_scan(sock, "wlp3s0", range.we_version_compiled, &head) < 0)
  {
    printf("Error during iw_scan. Aborting.\n");
    exit(2);
  }
SIOCGIFFLAGS

  /* Traverse the results */
  result = head.result;
  while (NULL != result)
  {
    printf("%s\n", result->b.essid);
    result = result->next;
  }

  exit(0);
}