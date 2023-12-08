## netlink
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <linux/netlink.h>
#include <linux/rtnetlink.h>
#include <net/if_arp.h>

#define MAX_PAYLOAD 1024

struct nl_request {
    struct nlmsghdr hdr;
    struct ifinfomsg ifi;
};

void process_netlink_response(char *buffer, ssize_t len) {
    struct nlmsghdr *nlh = (struct nlmsghdr *)buffer;

    while (NLMSG_OK(nlh, len)) {
        if (nlh->nlmsg_type == NLMSG_DONE) {
            break;
        }

        if (nlh->nlmsg_type == RTM_NEWLINK) {
            struct ifinfomsg *ifi = (struct ifinfomsg *)NLMSG_DATA(nlh);
            printf("Interface Index: %d\n", ifi->ifi_index);

            // Access additional attributes if needed
            struct rtattr *rta = IFLA_RTA(ifi);
            int rta_len = IFLA_PAYLOAD(nlh);

            while (RTA_OK(rta, rta_len)) {
                if (rta->rta_type == IFLA_IFNAME) {
                    printf("Interface Name: %s\n", (char *)RTA_DATA(rta));
                }
                // Add more attribute processing as needed

                rta = RTA_NEXT(rta, rta_len);
            }

            printf("------------------------\n");
        }

        nlh = NLMSG_NEXT(nlh, len);
    }
}

int main() {
    // Create a Netlink socket
   int sockfd = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
    if (sockfd == -1) {
        perror("socket");
        return 1;
    }

    // Prepare a Netlink request
    struct nl_request req;
    memset(&req, 0, sizeof(req));

    req.hdr.nlmsg_len = NLMSG_LENGTH(sizeof(struct ifinfomsg));
    req.hdr.nlmsg_type = RTM_GETLINK;
    req.hdr.nlmsg_flags = NLM_F_REQUEST | NLM_F_DUMP;
    req.hdr.nlmsg_seq = 1701882410;  // Your specific sequence number
    req.hdr.nlmsg_pid = 0;

    req.ifi.ifi_family = AF_UNSPEC;
    req.ifi.ifi_type = ARPHRD_NETROM;
    req.ifi.ifi_index = 0;
    req.ifi.ifi_flags = 0;
    req.ifi.ifi_change = 0;

    // Send the request using sendto
    if (sendto(sockfd, &req, req.hdr.nlmsg_len, 0, NULL, 0) == -1) {
        perror("sendto");
        close(sockfd);
        return 1;
    }

    // Receive the response using recvmsg
    char buffer[MAX_PAYLOAD];
    ssize_t len = recv(sockfd, buffer, sizeof(buffer), MSG_PEEK | MSG_TRUNC);
    if (len == -1) {
        perror("recv");
        close(sockfd);
        return 1;
    }

    // Process the Netlink response
    process_netlink_response(buffer, len);

    // Close the socket
    close(sockfd);

    return 0;
}
```


using iwlist to scan the network 
```c++
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

  /* Traverse the results */
  result = head.result;
  while (NULL != result)
  {
    printf("%s\n", result->b.essid);
    result = result->next;
  }

  exit(0);
}
```